use html5ever::parse_document; // , serialize
use html5ever::tendril::TendrilSink; // Default::default()).one needs this.
use markup5ever::interface::Attribute;
use markup5ever::{namespace_url, ns};
use markup5ever::{tendril::Tendril, LocalName, QualName}; // local_name,
use markup5ever_rcdom::{Handle, Node, NodeData, RcDom}; // , SerializableHandle
use std::cell::RefCell;
use std::rc::Rc;
// use tracing::{error, info}; // debug,

/// Parse html source in str into page node
/// eg; <html><body></body></html>
pub fn to_dom(source: &str) -> RcDom {
    parse_document(markup5ever_rcdom::RcDom::default(), Default::default()).one(source)
}

/// Parse parts of html in str into Nodes
/// eg; <div><div>contents</div></div>
pub fn to_dom_parts(html_text: &str) -> Vec<Rc<Node>> {
    // This inserts html_text into body.
    // eg;
    //  if html_text = "<div><div>contents</div></div>";
    //  becomes "<html><body><div><div>contents</div></div></body></html>"
    let parsed = to_dom(html_text);

    // get body
    let body_ptn = node_element("body", &vec![]);
    let body = child_match_first(&parsed.document, &body_ptn, true).unwrap();

    // take is essential to get elements deep in children recursively.
    // Otherwise clone does take shalow copy without its child elements.
    body.children.take()
}

pub fn attrs(attrs_vec: &Vec<(&str, &str)>) -> RefCell<Vec<Attribute>> {
    let mut attr_list: Vec<Attribute> = vec![];
    for (name, value) in attrs_vec {
        attr_list.push(attr(&name, &value));
    }
    RefCell::new(attr_list)
} // end of fn attrs

pub fn attr(name: &str, value: &str) -> Attribute {
    // ns!(html) for attribute name causes
    // WARN html5ever::serialize: attr with weird namespace Atom('http://www.w3.org/1999/xhtml' type=static)

    Attribute {
        name: QualName::new(
            None,
            ns!(), // <script type="text/javascript">
            LocalName::from(name),
        ),
        value: Tendril::from(value.to_string()),
    }
} // end of fn attr

pub fn node_element(ele_name: &str, attrs_vec: &Vec<(&str, &str)>) -> Rc<Node> {
    // QualName ns!() causes
    // WARN html5ever::serialize: node with weird namespace Atom('' type=static)

    Node::new(NodeData::Element {
        name: QualName::new(None, ns!(html), LocalName::from(ele_name)),
        attrs: attrs(&attrs_vec),
        template_contents: None.into(),
        mathml_annotation_xml_integration_point: false,
    })
} // end of fn node_element

pub fn node_text(contents: &str) -> Rc<Node> {
    let contents = Tendril::from(contents);
    let node_data = NodeData::Text {
        contents: RefCell::new(contents),
    };

    Node::new(node_data)
}

/// match_single: finish if one node matched.
fn node_child_match(
    node_obj: &Handle,
    node_ptn: &Node,
    node_list: &mut Vec<Handle>,
    recursive: bool,
    match_single: bool,
) {
    for child in node_obj.children.borrow().iter() {
        if element_match(child, node_ptn) {
            node_list.push(Rc::clone(child));
            if match_single {
                break;
            }
        }

        if recursive {
            node_child_match(child, node_ptn, node_list, recursive, match_single);
        }
    }
}

fn element_match(node: &Handle, node_ptn: &Node) -> bool {
    match node.data {
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            let name_obj = name;
            let attrs_obj = attrs;
            match node_ptn.data {
                NodeData::Element {
                    ref name,
                    ref attrs,
                    ..
                } => {
                    // both node and node_ptn are Element
                    if name_obj.local == name.local {
                        return attrs_match(attrs_obj.clone(), attrs.clone());
                    }
                    // names do not match
                    false
                }
                // node_ptn is not Element
                _ => false,
            }
        }
        // node is not Element
        _ => return false,
    }
}

fn attrs_match(attrs: RefCell<Vec<Attribute>>, attrs_ptn: RefCell<Vec<Attribute>>) -> bool {
    if 0 == attrs_ptn.borrow().len() {
        // no attrs condition
        // match always
        return true;
    }

    for att_for in attrs_ptn.borrow().iter() {
        'attrs: loop {
            for att in attrs.borrow().iter() {
                if att_for.name.local == att.name.local {
                    if att_for.value == att.value {
                        // att_for match att
                        // see next att_for
                        break 'attrs;
                    } else {
                        // value does not match
                        return false;
                    }
                }
            }
            // att_for.name does not exists in att
            return false;
        } // end of 'attrs: loop
    }

    // all of attrs_ptn match attrs
    true
}

pub fn child_match_list(
    node_obj: &Handle,
    node_ptn: &Node,
    recursive: bool,
    match_single: bool,
) -> Vec<Handle> {
    let mut node_list: Vec<Handle> = vec![];
    node_child_match(node_obj, node_ptn, &mut node_list, recursive, match_single);
    node_list
}

pub fn child_match_first(dom: &Handle, node_ptn: &Node, recursive: bool) -> Option<Handle> {
    // match_single stops seeking node_ptn if one node found,
    // it makes procedure faster.
    let match_single = true;
    let list = child_match_list(&dom, node_ptn, recursive, match_single);

    if list.len() < 1 {
        return None;
    }

    let child = Rc::clone(&list[0]);
    Some(child)
}

/// Return Rc<Node> of
/// <span id="page_json_str" style="display: none"></span>
pub fn span_json_new() -> Handle {
    // <span id="page_json_str" style="display: none">{"json":"json_data"}</span>
    let attrs = &vec![("id", "page_json_str")];
    node_element("span", &attrs)
}

/// Get span node from page_dom
/// <span id="page_json_str" style="display: none">{"json":"json_data"}</span>
pub fn get_span_json(page_node: &Rc<Node>) -> Option<Handle> {
    let ptn_span = span_json_new();
    child_match_first(&page_node, &ptn_span, true)
}

/// <script type="text/javascript" class="page_json">let page_json = {}</script>
pub fn get_script_json(page_node: &Rc<Node>) -> Option<Handle> {
    let attrs = &vec![("type", "text/javascript")];
    let script_ptn = node_element("script", &attrs);
    child_match_first(page_node, &script_ptn, true)
}

/// top_node <div id="page_top_node">
pub fn div_page_top_new() -> Handle {
    let attrs = &vec![("id", "page_top_node")];
    node_element("div", attrs)
}

pub fn get_div_page_top(page_node: &Rc<Node>) -> Option<Handle> {
    let ptn_top = div_page_top_new();
    child_match_first(page_node, &ptn_top, true)
}

// <div class="html subsectionContent">
pub fn div_subsection_content_new() -> Handle {
    let attrs = &vec![("class", "html subsectionContent")];
    node_element("div", &attrs)
}

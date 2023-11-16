pub fn contents() -> &'static str {
    // pub fn contents() -> [u8] {
    //     b####"

    // write the javascript code and put it into
    // r####"<javascript code>"####
    // in below

    r####"
// class Blox ele, let targetNext;
// it sometimes did refer old element that was deleted
// and element to be drawn before the targetNext was not shown
// but after targetNext gets
// value only contained in this.bxCenter().bxTop().ele() currently
// clearing ele procedure mignt not be needed,
// consider further on if some cleaning ele is needed

'use strict';

let page_json;

function bodyOnload () {
    // console.log("bodyOnload");
    
    page_json = page_json_read();

    const bxCenter = new BxCenter();
    
    const eleBloxTarget = document.createElement("div");
    document.body.appendChild(eleBloxTarget);
    
    const page = bxCenter.bxTop("Page");
    page.data(page_json);
    page.eleTarget(eleBloxTarget);
    page.eleDraw();

    page.menu().editorOpenListenerSet();
    
} // end of function bodyOnload;
    
// Get page string data from <span id="page_json_str">{...data...}</span>
// in HTML source .
// Return as javascript value .
function page_json_read () {
    // console.log("wc.js function page_json_read");

    let ele_page_json_str = document.getElementById("page_json_str");
    if(! ele_page_json_str){ return; }
    let page_json_str = ele_page_json_str.innerHTML;

    page_json_str = entityReferenceUnset(page_json_str);

    // Convert page_json_str (string) to javascript value .
    // And return it .
    // text \" becomes to value " .
    let f0 = new Function("return " + page_json_str + ";");
    return f0();
    
} // end of function page_json_read

function entityReferenceSet(str) {

    str = str.replace(/[<>&]/g, function(){
	let ref = ENTITY_REFERENCE[arguments[0]];
	
	if(ref){ return ref;}
	return arguments[0];
    });

    return str;

} // end of function entityReferenceSet

const ENTITY_REFERENCE = {
    '<' : '&lt;'
    , '>' : '&gt;'
    , '&' : '&amp;'
    // , '' : ''
};

// 
const REFERENCE_ENTITY = {
    '&lt;' : '<'
    , '&gt;' : '>'
    , '&amp;' : '&'
    // , '' : ''
};

// Replace charactors that can not be used in HTML and escaped .
//	'&lt;' : '<'
//	, '&gt;' : '>'
//	, '&amp;' : '&'
// Therefore \" must not replace to " since it still in text json data .
// \" will be handled as value '"' when put it into a javascript variable.
function entityReferenceUnset(str) {

    let re = /&[^;]+;/g;

    str = str.replace(
	//		return str.replace()
	re,
	function() {
	    let entity = REFERENCE_ENTITY[arguments[0]];
	    if(entity){ return entity;}
	    return arguments[0];
	}
    );

    return str;

} // end of function entityReferenceUnset

class BxCenter {
    // console.log("wc.js class BxCenter");

    bxTop() {
	// console.log("wc.js class BxCenter bxTop()");
	if(this.bxTopV == undefined && 0 < arguments.length){
	    const className = arguments[0];
	    if(className == undefined){ return; }
	    const key = "0";
	    this.bxTopV = this.bloxNew(undefined, className, key)
	}
	return this.bxTopV;
    } // end of class BxCenter bxTop 

    // counter for each blox to distinguish
    bloxCt = 0;

    // blox = this.bloxNew(parentBlox, className, key);
    bloxNew() {
	// console.log("wc.js class BxCenter bloxNew()");
	
	let className = arguments[1];

	// return class template as a function
	// if not exists return undefined
	const getClass = new Function(
	    "if(typeof " + className + "  != 'function'){ return; } "
		+
		" return " + className + " ;"
	);

	const theClass = getClass();
	if(! theClass){ return; }
	
	const blox = new theClass(...arguments);
	blox.bxCenter(this);
	blox.ct = this.bloxCt++;
	return blox;

    } // end of class BxCenter bloxNew 
    
} // end of class BxCenter end 

class Blox {
 
    constructor() {

	this.parentBx(arguments[0]);
	this.parent(arguments[0]);
	this.key(arguments[2]);
    } // end of class Blox constructor 

    bxCenter() {
	if(0 < arguments.length){ this.bxCenterV = arguments[0]; }
	return this.bxCenterV;
    } // end of class Blox bxCenter 
    
    thisIsBloxTop(){}

    parentBx() {
	// this.log("parentBx()");
	if(0 < arguments.length) {
	    this.parentBxV = arguments[0];
	}

	// return this.parentBxV;n
	return this.parentBxV;
    } // end of class Blox parentBx 

    // parent is a class instance just above this instalce
    // untill intorduce sub blox, parent and parentBx is same
    // if you intorduce sub blox, parent is just above and
    // parentBx is to hold sub blox and
    // parent is sometime as same as parentBx, somtime not
    parent() {
	// console.log("wc.js class Blox parent()");
	if(0 < arguments.length) { this.parentObj = arguments[0]; }
	return this.parentObj; 
   } // end of class Blox parent 

    key() {
	if(0 < arguments.length){ this.keyV = arguments[0]; }
	return this.keyV;
    } // end of class Blox key 

    data() {
	// console.log("wc.js class Blox data()");

	if(this.isBlank()){ return this.dataBlank(...arguments); }
	
	if(0 < arguments.length) { this.dataV = arguments[0]; }
	if(this.dataV){ return this.dataV; }

	if(this.parentBx().dataChild){
	    // return this.data(this.parentBx().dataChild(this));
	    const data = this.parentBx().dataChild(this);
	    if(data != undefined){
		return this.data(data);
	    }
	}
	
    } // end of class Blox data 

    // data used in operation not stored
    currentStatus() {
	if(this.currentStatusV == undefined){ this.currentStatusV = {}; }
	return this.currentStatusV;
    } // end of class Blox currentStatus 
    
    eleTarget() {
	// this.log("eleTarget()");

	// eleTarget set by argument
	if(0 < arguments.length) { this.eleTargetV = arguments[0];}
	if(this.eleTargetV){ return this.eleTargetV; }

	// find eleTarget by targetKey
	let eleTop;
	if(this != this.bxCenter().bxTop()){
	    eleTop = this.bxCenter().bxTop().eleTarget();
	}
	
	if(eleTop){
	    // querySelector requires "." inside of the name to be escaped "\."
	    let targetName = this.eleTargetName().replace(/\./g, "\\.");
	    const eleTarget = eleTop.querySelector("."+targetName);

	    if(eleTarget){
		// return this.eleTargetV;
		return eleTarget;
	    }
	}

	// ask eleTarget to parentBx()
	if(this.parentBx()){
	    const eleTarget = this.parentBx().eleTargetChild(this);
	    if(eleTarget){
		// return this.eleTargetV;
		return eleTarget;
	    }
	}

    } // end of class Blox eleTarget 

    eleTargetName() {
	return this.bloxAddress() + "Target";
    } // end of class Blox eleTargetName 
    
    // if blox was moved, this.bloxAddress needs to be refreshed
    // and its children as well
    // to avoid repetitions of bloxAddress() each time,
    // better to keep the values in the method
    // then, the procedure following is done that is good for both perpose
    // at class Blox eleDraw(), this.bloxAddress(undefined)
    keyDelimiter = "._.";
    classDelimiter = ".__.";
    bloxAddress() {
	// this.log("bloxAddress");

	let address = "";
	if(this.parentBx()){
	    address += this.parentBx().bloxAddress() + this.classDelimiter;
	}
	address += this.constructor.name + this.keyDelimiter + this.key();
	return address;

    } // end of class Blox bloxAddress 

    bloxPrefix(name) {
	if(name == undefined){ return this.bloxAddress(); }
	return this.bloxAddress() + name;
    } // end of class Blox bloxPrefix 

    // ele.querySelector(selector)
    // selector should escape . to \.
    bloxPrefixEscaped(name) {
	const str = this.bloxPrefix(name);
	return str.replace(/\./g, "\\.");
    } // end of class Blox bloxPrefixEscaped 

    bloxAddressFromEle() {
	// this.log("bloxAddressFromEle()");

	let ele = arguments[0];
	while(ele){
	    let bxAddress = ele.getAttribute("data-bxAddress");
	    if(bxAddress){ return bxAddress; }
	    ele = ele.parentNode;
	}

    } // end of class Blox bloxAddressFromEle 

    bloxByAddress() {
	// this.log2("bloxByAddress()");

	let blox;
	let addresses = arguments[0].split(this.classDelimiter);

	for(const address of addresses){
	    const [className, key] = address.split(this.keyDelimiter);

	    // only the first time
	    if(blox == undefined){
		// blox = this.bxCenter().bxTop();
		blox = this.bxCenter().bxTop();
		if(blox.constructor.name != className){
		    return; }
		continue;
	    }
	    
	    // CAUTION: Do not use bloxChildNew,
	    // it might create bloxes and use memory
	    // by client request with address given
	    // that might cause problems
	    blox = blox.bloxChild(className, key);
	    if(blox == undefined){ return; }
	}
	
	return blox;
	
    } // end of class Blox bloxByAddress 

    bloxFromElePart() {
	// this.log2("bloxFromElePart()");
	const ele = arguments[0];
	let bxAddress = this.bloxAddressFromEle(ele);
	return this.bloxByAddress(bxAddress);
    } // end of class Blox bloxFromElePart 

    querySelectorBx(ele, name) {
	const selectors = this.bloxPrefixEscaped(name);
	return ele.querySelector("." + selectors);
    } // end of class Blox querySelectorBx 
    
    querySelectorAllBx(ele, name) {
	const selectors = this.bloxPrefixEscaped(name);
	return ele.querySelectorAll("." + selectors);
    } // end of class Blox querySelectorAllBx 
    
    eleTargetChild() {} // end of class Blox eleTargetChild 

    // To draw a node, git it as a parameter, eg: this.ele(ele_node);
    // To get node that has been drawn, call this without parameter,
    // eg : ele_node = this.ele();
    // To delete node drawn, use undefined as a parameter
    // eg: this.ele(undefined);
    // if Blox.eleV has a node value, it was drawn,
    // otherwise nothing drawn of this in the page.
    ele() {
	// this.log("ele()");	

	if(arguments.length == 0) { return this.eleV; }

	let ele = arguments[0];

	// delete current ele
	if(this.eleV){
	    let eleDrawn = this.eleV;
	    if(eleDrawn.parentNode.contains(eleDrawn)){
		eleDrawn.parentNode.removeChild(eleDrawn);
		// once removed eleDraw, its children work any more
		this.childrenEleClear();
	    }
	    delete this.eleV;
	}

	// Clear ele drawn
	// An argument was given but it is undefined
	if(! ele){
	    // to avoid to draw its children
	    // based on element that is under this.eleV deleted
	    // delete all elements under this element
	    this.childrenEleClear();
	    return;
	}

	ele.setAttribute("data-bxAddress", this.bloxAddress());

	let targetNext;
	if(this.targetNext){
	    const targetNext2 = this.targetNext();
	    // confirm if this.targetNext() exits as of
	    // this.bxCenter().bxTop().ele()
	    // do not use targetNext that was deleted and no longer in the ele
	    // it can not be confirmed on this.eleV value
	    // because some ele is not of document yet
	    if(this.bxCenter().bxTop().ele().contains(targetNext2)){
		targetNext = targetNext2;
	    }
	}

	if(targetNext) {
	    targetNext.parentNode.insertBefore(ele, targetNext);
	} else  {
	    const eleTarget = this.eleTarget();
	    if(eleTarget == undefined){
		// DBG
		// it makes an err that may show err debug monitor
		a = b;
		
		return;
	    }
	    
	    eleTarget.appendChild(ele);
	}

	this.eleV = ele;
	return this.eleV;
	
    } // end of class Blox ele 

    eleDraw() {
	// this.log2("eleDraw()");

	// if no method eleDrawInst exists, the class is not to be drawn
	if(! this.eleDrawInst){ return; }

	// skip drawn
	if(this.bloxDrawn()){
	    // this.log2("eleDraw() skip");
	    return; }

	// this makes possible to record what child drawn
	// and skip drawing if already drawn
	// if this.eleDraw() is called directory,
	// this.parentBx().bloxChildDrawn2() is empty
	// in such case it does not check and skip whether it has been drawn
	// otherwise, this.eleDraw() is called by bloxChildEleDraw of
	// this.parentBx().eleDraw() and check if it already drawn
	// set {} at here means it records what drawn in this.eleDrawInst
	this.bloxChildDrawn2({});

	// // if set this.bloxDrawn({"drawn": true}) at here
	// // it records what child drawn at this.eleDrawInst
	// // if set this.bloxDrawn({"drawn": true}) after this.eleDrawInst
	// // it can not record what child drawn at this.eleDrawInst
	// this.bloxDrawn({"drawn": true});

	this.bloxDrawn({"drawn": true});
	this.bloxAddress(undefined);
	// this.childrenEleClear();

	// Method eleDrawInst is to be coded in class that extends Blox.
	// Each of those classes have specific method.
	this.eleDrawInst(...arguments);

	// skip drawing editor if it is not set in menu.currentBlox()
	// do not call this.blox() that create an editor instance
	const editor = this.bloxChild(this.constructor.name + "Editor", 0);
	// editor.menu() is undefined if it is not opened
	if(editor){
	    let drawTheEditor = false;
	    if(editor.menu()){
		if(editor.menu().currentBlox() == this){
		    drawTheEditor = true;
		}		
	    }

	    if(! drawTheEditor){
		editor.bloxDrawn({"drawn": true});
	    }
	}

	this.bloxChildEleDraw(...arguments);

	// clear
	this.bloxChildDrawn2(undefined);

    } // end of class Blox eleDraw 

    htmlPhReplace(html, htmlApply) {
	// console.log("wc.js class Blox htmlPhReplace()");
	
	if(htmlApply){
	    html = html.replace("<!--placeHolder-->", htmlApply);
	}
	return html;
    } // end of class Blox htmlPhReplace 

    // Convert html to ele nodes
    // If there are plural elements on top layer,
    // makes a one top dev element and append those to it
    // to make top level elemant always one .
    eleFromHtml() {
	
	let ele = document.createElement('div');
	
	let html = arguments[0];
	if(html == undefined){ return ele; }

	html = this.htmlPrefixSet(html);

	ele.innerHTML = html.trim();
	if(ele.childNodes.length == 1){
	    ele = ele.childNodes[0];
	}

	return ele;

    } // end of class Blox eleFromHtml 

    htmlPrefixSet(html) {
	// console.log("wc.js class Blox htmlPrefixSet()");

	// <input type="button" class="{BXPF=editorEnter}" value="Enter">
	const that = this;
	let htmlRep = html.replace(/{BXPF=(.[^}]+)}/g, function(){
	    if(arguments[1] == undefined){
		return arguments[0];
	    }
	    return that.bloxPrefix(arguments[1]);
	});
	
	return htmlRep;
	
    } // end of class Blox htmlPrefixSet 

    // this.eleVisibleSet (eleArg, req);
    // req: {'class0', 1, 'class1': 0};
    // class0: class name;
    // 1: show, 0: off;
    eleVisibleSet(eleArg, req) {
	// this.log("eleVisibleSet");
	// console.log("wc.js class Blox eleVisibleSet()");
	
	if( ! eleArg){ return;}
	// const eleArg = this.ele();
	for(const key0 in req){
	    let elePart = this.querySelectorBx(eleArg, key0);
	    if( ! elePart){ continue;}
	    if(req[key0]){
		elePart.classList.remove('invisible');
	    }else{
		elePart.classList.add('invisible');
	    }
	}

    } // end of class Blox eleVisibleSet

    // all bloxChild will be drawn,
    // but blox that has empty method of eleDrawInst() does nothing
    // eleDrawInst(){}

    // drawn = this.bloxDrawn();
    // this.bloxDrawn({"drawn": true});
    bloxDrawn() {
	// this.log("bloxDrawn()");
	
	let option = arguments[0];
	if(option == undefined){ option = {}; }

	// ask parent if this was already drawn
	const parent = this.parentBx();
	if(parent == undefined){ return; }

	// when this.eleDraw() is not called by its parent
	// parent.bloxChildDrawn2() should not have its value
	// in such case it is not applicable to check if it has been drawn
	let parentDrawn = parent.bloxChildDrawn2();
	if(parentDrawn == undefined){ return; }

	const drawnClass = parentDrawn[this.constructor.name];
	if(option.drawn){
	    if(drawnClass == undefined){
		parentDrawn[this.constructor.name] = {};
	    }
	    parentDrawn[this.constructor.name][this.key()] = this;
	} else {
	    if(drawnClass == undefined){ return; }
	    
	    if(parentDrawn[this.constructor.name][this.key()]){ return true; }
	}
	
	return false;

    } // end of class Blox bloxDrawn 

    // this.bloxChildDrawn2({});
    // drawn = this.bloxChildDrawn2();
    bloxChildDrawn2() {
	// this.log("bloxChildDrawn2()");
	if(0 < arguments.length){ this.bloxChildDrawn2V = arguments[0]; }
	return this.bloxChildDrawn2V;
    } // end of class Blox bloxChildDrawn2 
    
    // store bloxes and return blox stored
    // option: {"create": true} / {"remove": true} / {"put": blox}
    // blox = this.bloxChild(className, key);
    // blox = this.bloxChild(className, key, {"create": true});
    // this.bloxChild(className, key, {"remove": true});
    bloxChild(className, key, option) {
	// this.log("bloxChild()");

	if(this.bloxChildV == undefined){ this.bloxChildV = {}; }

	if(arguments.length == 0){ return this.bloxChildV; }

	if(className == undefined){ return; }

	if(arguments.length == 1){ return this.bloxChildV[className]; }
	
	if(key == undefined){ return; }

	if(this.bloxChildV[className] == undefined){
	    if(option && ! option.create && ! option.put){ return; }
	    this.bloxChildV[className] = {};
	}
	let classChildren = this.bloxChildV[className];

	// if no option given, return it whatever if it exists
	if(arguments.length == 2){ return classChildren[key]; }

	// better to change option.dlete to option.remove
	// because it does not delete the instance itself
	if(option.remove){
	    
	    classChildren[key].ele(undefined);

	    // do not this.childrenEleClear() here
	    // reason 1) blox.ele(undefined) call also blox.childrenEleClear()
	    // reason 2) want to leave blox.childrenEleClear() at blox.eleb
	    // because there is a cese want to remove only ele, but not blox
	    
	    delete classChildren[key];

	    if(key == this.idBlank){ this.bloxChildBlankBuff(undefined); }
	    
	    return;
	}

	// already exists
	if(classChildren[key]){ return classChildren[key]; }

	// not exits and not create
	if(! option){ return; }

	if(option.create){
	    let blox;
	    if(key == this.idBlank){
		blox = this.bloxChildBlankBuff(...arguments);
	    } else {
		blox = this.bxCenter().bloxNew(this, ...arguments);
	    }
	    
	    if(blox){ classChildren[key] = blox; }
	}

	if(option.put){
	    classChildren[key] = option.put;
	    classChildren[key].parentBx(this);
	}

	return classChildren[key];
	
    } // end of class Blox bloxChild 

    // blox = this.bloxChildBlankBuff(className);
    // blox = this.bloxChildBlankBuff();
    // this.bloxChildBlankBuff(undefined);
    bloxChildBlankBuff() {
	// this.log("bloxChildBlankBuff()");

	if(0 < arguments.length){
	    if(arguments[0] == undefined){
		delete this.bloxChildBlankBuffV;
		return;
	    }
	}
	
	if(this.bloxChildBlankBuffV == undefined){
	    const className = arguments[0]
	    this.bloxChildBlankBuffV =
		this.bxCenter().bloxNew(this, className, this.idBlank);
	}
	
	return this.bloxChildBlankBuffV;

    } // end of class Blox bloxChildBlankBuff

    // delete previous blank blox and create a new blank blox
    bloxChildBlankNew(className) {
	const blox = this.bloxChild(className, this.idBlank);
	if(blox){
	    this.bloxChild(className, this.idBlank, {"remove" : true});
	}
	return this.bloxChild(className, this.idBlank, {"create" : true});
    } // end of bloxChildBlankNew

    // return the current blank blox, or create new blank blox
    bloxChildBlank(className) {
	return this.bloxChild(className, this.idBlank, {"create" : true});
    } // end of bloxChildBlank

    // this.bloxChildRemove(child);
    bloxChildRemove(child) {

	const className = child.constructor.name;
	this.bloxChild(className, child.key(), {"remove": true});

    } // end of class Blox bloxChildRemove 

    // this.bloxRemove();
    bloxRemove() {

	const parent = this.parentBx();
	if(parent == undefined){ return; }
	parent.bloxChildRemove(this);

    } // end of class Blox bloxRemove 

    // remove all children that have same class name
    // this.childRemoveSibling(child);
    childRemoveSibling(child) {
	// this.log("childRemoveSibling()");

	const bloxChild = this.bloxChild();

	for(let className of Object.keys(bloxChild)){
	    if(className != child.constructor.name){ continue; }
	    for(let key of Object.keys(bloxChild[className])){
		const child = bloxChild[className][key];
		child.clear();
	    }
	}

    } // end of class Blox childRemoveSibling 

    childrenEleClear() {
	// this.log("childrenEleClear()");
	const bloxChild = this.bloxChild();

	for(let className of Object.keys(bloxChild)){
	    for(let key of Object.keys(bloxChild[className])){
		const blox = bloxChild[className][key];
		blox.ele(undefined);
		blox.childrenEleClear();
	    }
	}
    } // end of class Blox childrenEleClear 

    // show child list
    // this.dbgBloxChildMonitor()
    dbgBloxChildMonitor() {
	// this.log("dbgBloxChildMonitor()");
	// console.log("wc.js class Blox dbgBloxChildMonitor()");
	const bloxChild = this.bloxChild();
	for(const className in bloxChild){
	    for(const key in bloxChild[className]){
		const blox = bloxChild[className][key];
	    }
	}
	
    } // end of class Blox dbgBloxChildMonitor 

    editor() {
	const className = this.constructor.name + "Editor";
	return this.bloxChild(className, "0", {"create" : true});
	// const editor = this.bloxChild(className, "0", {"create" : true});
	// return editor;
    } // end of class Blox editor 
    
    bloxChildEleDraw() {
	// this.log2("bloxChildEleDraw()");

	const bloxChild = this.bloxChild();
	for(let name of Object.keys(bloxChild)){
	    for(let key of Object.keys(bloxChild[name])){

		// this.log2("bloxChildEleDraw()", name + ", " + key);
		
		bloxChild[name][key].eleDraw();
	    }
	}

    } // end of class Blox bloxChildEleDraw 

    idBlank = "blank";
    
    isBlank() {
	if(this.key() == this.idBlank){ return true; }

	const parent = this.parentBx();
	if(parent){
	    if(parent.isBlank()){ return true; }
	}

    } // end of class Blox isBlank 

    dataBlank() {
	if(0 < arguments.length){ this.dataBlankV = arguments[0]; }
	if(this.dataBlankV == undefined){ this.dataBlankV = {}; }
	return this.dataBlankV;	
    } // end of class Blox dataBlank 
   
    methodInstance() {
	const fname = arguments[0];
	const fcode = new Function("return this."+fname+";");
	return fcode.apply(this);
    } // end of class Blox methodInstance 

    classAndKey() {
	return this.constructor.name + "." + this.key()+ "(" + this.ct +")";
    } // end of class Blox classAndKey 

    parentThisInfo() {
	if(this.parentBx()){
	    return this.parentBx().classAndKey() + "-" + this.classAndKey();
	} else {
	    return "(no parent)-" + this.classAndKey();
	}
    } // end of class Blox parentThisInfo 

    // this.move(bloxTo, moveOption);
    move(bloxTo) {
	// this.log("move()");

	let option = arguments[1];

	// remove from parent
	this.parentBx().bloxChild(this.constructor.name, this.key(), {"remove" : true});

	// children ele might be used as targetNext or some target
	// that is not exists any more
	// in that case, element drawn on the target can not be shown
	this.childrenEleClear();
	
	const name = this.constructor.name;
	const key = this.key();

	if(option.child){
	    bloxTo.bloxChild(name, key, {"put": this});
	}
	else {
	    bloxTo.parentBx().bloxChild(name, key, {"put": this});
	}

    } // end of class Blox move

    clear() {

	const parent = this.parentBx();
	if(parent == undefined){ return; }

	this.ele(undefined);
	this.childrenEleClear();
	parent.bloxChildRemove(this);

    } // end of class Blox clear 

    log() {
	log(this, ...arguments);
    } // end of class Blox log 

    // this.log2("log2", "");
    log2() {

	let message = "wc.js log2 " + this.bloxTreeNameKey() +":"+arguments[0];
	if(1 < arguments.length){
	    message += "\n" + arguments[1];
	}

	 console.log(message);

	// console.log("wc.js log2 " + this.bloxTreeNameKey() +":"+arguments[0] + "\n" + arguments[1]);
	// console.log("wc.js log2 mssg:" + arguments[1]);

    } // end of class Blox log2 

    bloxTree() {
	if(this.bloxTreeV == undefined){

	    const tree = [this];
	    let parent;
	    if(this.parentBx){
		parent = this.parentBx();
	    }
	    
	    while(parent){
		tree.push(parent);
		if(parent.parentBx){
		    parent = parent.parentBx();
		} else {
		    parent = undefined;
		}
	    }
	    
	    this.bloxTreeV = tree.reverse();
	    
	}
	
	return this.bloxTreeV;
	
    } // end of class Blox bloxTree 

    bloxTreeNameKey() {
	const treeNameKey = []
	for(const blox of this.bloxTree()){
	    treeNameKey.push(
		blox.constructor.name
		    +"."+blox.key()
		    +"("+blox.ct+")"
	    );
	}
	return treeNameKey.join("-");
    } // end of class Blox bloxTreeNameKey 
    
    name() {
	if(0 < arguments.length){
	    this.nameV = arguments[0];
	    return this.nameV;
	}
	return this.bloxChild("Name", "0", {"create" : true});
	return this.nameV;
    } // end of class Blox name 
    
} // end of class Blox end 

// enter the method name of you yous this.log inside of
// at first
// this.log("methodName() YourMessage");
function log() {
    // console.log("function log");

    const objArg = arguments[0];
    const message = arguments[1];

    const tree = [objArg];
    let parent;
    if(objArg.parentBx){
	parent = objArg.parentBx();
    }
    
    while(parent){
	tree.push(parent);
	if(parent.parentBx){
	    parent = parent.parentBx();
	} else {
	    parent = undefined;
	}
    }

    const tree2 = tree.reverse();
    const tree3 = [];
    for(const obj of tree2){
	tree3.push(obj.constructor.name +"."+obj.key());
    }
    
    const treeText = tree3.join("-");
    
    console.log("log:" + treeText + "(" + objArg.ct +") "+ message);
    
} // end of function log

class Page extends Blox {

    dataChild(child) {

	// ItemsCenter
	if(child.constructor.name == "ItemsCenter"){
	    return this.data().data.subsection;
	}
	
    } // end of class Page dataChild 
    
    eleDrawInst() {
	// this.log("eleDrawInst()");

	const ele = document.createElement("div");
	// ele.appendChild(document.createTextNode("Page"));
	
	// menu
	const eleMenu = document.createElement("div");
	eleMenu.classList.add(this.menu().eleTargetName());
	ele.appendChild(eleMenu);
	
	// navi
	const eleNavi = document.createElement("div");
	eleNavi.classList.add(this.navi().eleTargetName());
	ele.appendChild(eleNavi);
	
	// Index target
	const eleIndex = document.createElement("div");
	eleIndex.classList.add(this.item().index().eleTargetName());
	ele.appendChild(eleIndex);

	// Conents target
	const eleContents = document.createElement("div");
	eleContents.classList.add(this.item().contents().eleTargetName());
	ele.appendChild(eleContents);

	// navi footer
	const eleFooter = document.createElement("div");
	ele.appendChild(eleFooter);

	this.ele(ele);

	this.menu().elePageTop(this.ele());
	this.menu().bloxDrawn({"drawn": true});

	this.item().index().eleDraw();
	this.item().bloxDrawn({"drawn": true});

	const itemChildren = this.item().children();
	for(const child of itemChildren){
	    child.eleDraw();
	}

	// open indexItem editor if not item exists
	if(itemChildren.length == 0){ this.indexItemBlankEditor(); }

	// footer navi
	if(0 < itemChildren.length){
	    const contents = itemChildren[0].contents();
	    eleFooter.appendChild(contents.eleNavi());
	}

    } // end of class Page eleDrawInst 
    
    itemsCenter() {
	return this.bloxChild("ItemsCenter", "0", {"create" : true});;
    } // end of class Page itemsCenter 

    // item top
    item() {
	const item = this.bloxChild("Item", "0", {"create" : true});
	item.itemsCenter(this.itemsCenter());
	return item;
    } // end of class Page item 

    menu() {
	if(this.menuV == undefined){
	    this.menuV = this.bloxChild("Menu", "0", {"create" : true});
	}
	return this.menuV;
    } // end of class Page menu 

    navi() {
	const navi = this.bloxChild("Navi", "0", {"create" : true});
	navi.dataParent(this.data().data);
	return navi;	
    } // end of class Page navi
    
    indexItemBlankEditor() {
	// this.log2("indexItemBlankEditor()");
	
	const indexItem = this.item().indexItem()
	indexItem.editor().currentStatus().editOption = "child";
	indexItem.editor().editorInsert(indexItem);

	// this is not opened throw this.menu().editorOpen(),
	// open this.menu() manually
	this.menu().eleDrawInst();
	
    } // end of indexItemBlankEditor

} // end of class Page end 

class Editor extends Blox {

    ele(){
	// console.log("wc.js class Editor ele()");
	
	if(arguments[0] != undefined){
	    // this.setErrMessage(...arguments);
	    this.setMessage(...arguments);
	    this.result("undefined");
	    // this.setEvent(...arguments);
	    this.setEvent2(...arguments);
	}
	
	return super.ele(...arguments);
	
    } // end of class Editor ele 

    // // this editorClick() was duplicated
    // editorClick() {
    // 	// this.log("editorClick()");
    // 	const event = arguments[0];
    // 	const name = arguments[1];

    // 	event.stopPropagation();
    // 	event.preventDefault();
	
    // 	// this.log("editorClick() name:" + name);

    // 	const fname = "editor"+name;
    // 	const fcode = new Function("return this."+fname+";");
    // 	const finst = fcode.apply(this);
    // 	if(finst == undefined){ return; }
    // 	finst.apply(this, [...arguments]);	

    // }
    // end of class Editor editorClick 

    // setEvent(ele, menu) {
    setEvent(ele) {
	// this.log("setEvent()");
	// console.log("wc.js class Editor setEvent()");
	
	// if(ele == undefined){ return; }
	for(const name of this.menuItem){
	    let fname = "editor"+name;
	    const eleSws = this.querySelectorAllBx(ele, fname);
	    if(eleSws.length == 0){ continue;}

	    const fcode = new Function("return this.editor"+name+";");
	    const finst = fcode.apply(this);
	    if(finst == undefined){ continue; }
	    
	    const obj = this;
	    this.log("setEvent() name: " +name+" obj.menu():" + obj.menu());
	    for(const eleSw of eleSws){
		eleSw.addEventListener('click', function(event){
		    // event.stopPropagation(); // prevent
		    finst.apply(obj, [event]);
		} );
	    }
	}

    } // end of class Editor setEvent  
    
    menuItem = [
	'Cancel', 'Enter'
	, "MoveOpen", "InsertOpen"
	, 'InsertMenuBefore', "InsertMenuAfter", "InsertMenuChild"
	, 'NewPage', 'Subcontent'
	, 'Delete'
	, 'DeleteExecute'
	// , 'Eventindividual'
    ];

    // changed / err / warning
    //
    // this.result("undefined");
    // 
    // changed
    // this.result("changed", true);
    // this.result("changed", false);
    // const changed = this.result("changed");
    // 
    result() {

	// clear
	if(arguments[0] == "undefined"){ this.resultV = undefined; }

	// ini
	if(this.resultV == undefined){
	    this.resultV = {};
	    // this.resultV.changed
	    this.resultV.err = [];
	    this.resultV.warning = [];
	}

	if(arguments.length == 0){ return this.resultV; }

	// changed 0: "changed" (keyword), 1: boolean
	if(arguments[0] == "changed"){
	    if(arguments.length == 1){ return this.resultV.changed; }
	    if(arguments[1]){
		this.resultV.changed = true;
	    } else {
		this.resultV.changed = false;
	    }
	    const menu = this.menu();
	    if(menu){
		if(this.resultV.changed){ this.menu().changed(true); }
	    }
	    return this.resultV;
	}

	// err or warning
	let name;
	if(arguments[0] == "err" || arguments[0] == "warning"){
	    name = arguments[0];
	    if(arguments.length == 1){ return this.resultV[name]; }
	    this.resultV[name].push(arguments[1]);
	    return this.resultV;
	}
	
    } // end of class Editor result 

    setMessage() {
	// this.log("setMessage()");
	
	this.setErrMessage(...arguments);

	const ele = arguments[0];
	const eleMessage = this.querySelectorBx(ele, "message");
	if(eleMessage == undefined){ return; }
	const message = document.createElement('div');
	message.appendChild(document.createTextNode(this.parentThisInfo()));
	eleMessage.appendChild(message);
	
    } // end of class Editor setMessage 
    
    setErrMessage() {
	// this.log("setErrMessage()");
	
	const ele = arguments[0];

	let err = this.result("err");
	
	// this.log("setErrMessage() err.length:" + err.length);
	
	if(0 < err.length){
	    const eleMessage = this.querySelectorBx(ele, "message");
	    const errMessage = document.createElement('span');
	    errMessage.appendChild(document.createTextNode(err.join("/")));
	    errMessage.setAttribute("style", "color: red");
	    eleMessage.appendChild(errMessage);
	}
	
    } // end of class Editor setErrMessage 

    // Set action on buttons of ele given as a parameter adding event listener.
    // menuItem is a list of keyword
    // fname made from keywords, eg.
    // in case of key word is "Enter",
    // fneme `editorEnter` (editor + keyword "Enter")
    // that can be found in html as follows.
    // <input type="button" class="{BXPF=editorEnter}" value="Enter"> 
    // When the button is clicked,
    // method "editorEnter" of the class will be called.
    setEvent2() {
	const ele = arguments[0];
	for(const name of this.menuItem){
	    let fname = "editor"+name;
	    const eleSws = this.querySelectorAllBx(ele, fname);
	    if(eleSws.length == 0){ continue;}
	    const that = this;
	    for(const eleSw of eleSws){
		eleSw.addEventListener('click', function(event){
		    that.editorClick.apply(that, [event, name]);
		} );
	    }
	}
    } // end of class Editor setEvent2 

    // Action when an editor buttons was clicked.
    // This event listener is set by method setEvent2().
    editorClick() {
	// this.log("editorClick()");
	const event = arguments[0];
	const name = arguments[1];

	event.stopPropagation();
	event.preventDefault();
	
	// this.log("editorClick() name:" + name);

	const fname = "editor"+name;
	const fcode = new Function("return this."+fname+";");
	const finst = fcode.apply(this);
	if(finst == undefined){ return; }
	finst.apply(this, [...arguments]);	

    } // end of class Editor editorClick 

    htmlEditorBox = (`
<table  class="editTable">
	<tr>
	  <td colspan=2 class="{BXPF=message}"</td>
	</tr>
	<!--placeHolder-->
</table>
`); // end of class Editor htmlEditorBox 
    
    htmlEditorEnter = (`
        <tr class="{BXPF=htmlEditorEnter}">
	  <td></td>
      <td>
	    <input type="button" class="{BXPF=editorEnter}" value="Enter"> 
	    <input type="button" class="{BXPF=editorCancel}" value="Cancel">
	    <input type="button" class="{BXPF=editorNewPage}" value="New Page">
	  </td>
	</tr>
<!--placeHolder-->
`); // end of class Editor htmlEditorEnter

    htmlEditorTitleHref = (`
	<tr>
	  <td>title</td>
	  <td><input class="{BXPF=inputTitle}"></td>
	</tr>
	<tr>
	  <td>href</td>
	  <td>
	    <input class="{BXPF=inputHref}" value="#">
	  </td>
	</tr>
<!--placeHolder-->
`); // end of class Editor htmlEditorTitleHref
    
    htmlEditorInter = (`
	<tr>
	  <td></td>
	  <td><hr class="editor_hr"></td>
	</tr>

      
	<tr class="{BXPF=editorMoveMenu}">
	  <td></td>
	  <td>
	    <div class="editorMoveMenu">
            <input type="button" class="{BXPF=editorMoveOpen}" value="Move"> 
            <input type="button" class="{BXPF=editorInsertOpen}" value="Insert"> 
	    /
	    <input type="button" class="{BXPF=editorDelete}" value="Delete">

	    </div>
          </td>
	</tr>

	<tr class="{BXPF=editorDeleteConfirm} invisible testColor">
	  <td></td>
	  <td>
	    <div>
	      Delete , sure ?
	      <input type="button" class="{BXPF=editorDeleteExecute}" value="Execute">
	      <input type="button" class="{BXPF=editorCancel}" value="Cancel">
	    </div>
          </td>
	</tr>
<!--placeHolder-->
`); // end of class Editor htmlEditorInter 

    htmlEditorTargetSelect = (`
<tr>
<td>Select where 
<span class="{BXPF=editorSelectType}"></span>
 to 
<span class="{BXPF=editorSelectOption}"></span>
</td>
<td></td>
</tr>

<tr>
<td>
<input type="button" class="{BXPF=editorCancel}" value="Cancel">
<input type="button" class="{BXPF=editorOptionSetChild}" value="as Child">
<input type="button" class="{BXPF=editorOptionSetBefore}" value="Before">
<input type="button" class="{BXPF=editorOptionSetAfter}" value="After">
</td>
<td></td>
</tr>
<!--placeHolder-->
 `); // end of class Editor htmlEditorTargetSelect 
    
    htmlEditorTextarea = (`
<tr class="editor_subsection_content_type">
	  <td></td>
	  <td>
	    <input type="button" value="B" class="textareaBigger">
	  </td>
	</tr>

	<tr>
	  <td></td>
	  <td><textarea class="{BXPF=editorContent} textareaBig"></textarea></td>
	</tr>
	<!--placeHolder-->
	`); // end of class Editor htmlEditorTextarea 
    

    htmlEditorContentType = (`
<tr class="editor_subsection_content_type">
	  <td></td>
	  <td>
<label><input type="radio" name="{BXPF=contentType2}" value="html" class="{BXPF=contentTypeHtml}" />HTML</label>
<label><input type="radio" name="{BXPF=contentType2}" value="text" class="{BXPF=contentTypeText}" />Text</label>
<label><input type="radio" name="{BXPF=contentType2}" value="script" class="{BXPF=contentTypeScript}" />Script</label>

	  </td>
	</tr>
	<!--placeHolder-->
 `); // end of class Editor htmlEditorContentType 

    // 	= (`
    // `); // end of

    menu() {
	// this.log("menu()");

	const page = this.bxCenter().bxTop();
	return page.menu();
	
	// if(0 < arguments.length){ this.menuV = arguments[0];}
	// return this.menuV;
    } // end of class Editor menu 
    
    editorCancel() {
	// this.log("editorCancel()");
	this.editorClose(...arguments);
    } // end of class Editor editorCancel 

    editorClose(event) {
	// this.log("editorClose()");

	// this editorClose() is called by eventListener with event argument
	// this editorClose() is also called by this.menu().editorOpen()
	// without event
	if(event){
	    event.stopPropagation(); // prevent
	}

	if(this.currentStatus().editorInter){ this.editorInterOff(); }

	this.ele(undefined);
	this.menu().editorClose();

    } // end of class Editor editorClose 

    editorEnter() {
	// this.log2("editorEnter()", "");
	
	if(this.result("changed")){
	    this.menu().changed(true);
	}

	if(this.result().err.length == 0){
	    this.editorClose(...arguments);
	} else {
	    // you might apply changes on display
	    // even some err happened
	    // drawing a element that editing on mignt also draw the editor
	    // so do not call re-drawing editor at here
	    // this.eleDraw();
	}
	
    } // end of class Editor editorEnter 
    
    // editorMoveOpen(event) {
    editorMoveOpen() {
	// this.log("editorMoveOpen()");

	this.currentStatus().editType = "move";

	if(! this.currentStatus().editOption){
	    this.currentStatus().editOption = "after"; }

	this.editorInterOn();
	
	this.eleDrawInter();
	
    } // end of class Editor editorMoveOpen

    editorInsertOpen(event) {
	// this.log("editorInsertOpen()");

	this.currentStatus().editType = "insert";
	this.currentStatus().editOption = "after";

	this.editorInterOn();
	
	this.eleDrawInter();
	
    } // end of class Editor editorInsertOpen 

    editorInterOn() {
	this.interListenerSet(this.parentBx());
	this.currentStatus().editorInter = true;

    } // end of class Editor editorInterOn 

    editorInterOff() {
	// this.log("editorInterOff()");
	
	this.interListenerRemove(this.parentBx());
	delete this.currentStatus().editorInter;
	delete this.currentStatus().editType;
	delete this.currentStatus().editOption;
	
    } // end of class Editor editorInterOff 

    // Set an event listener on the whole page to select a target.
    interListenerSet() {
	// this.log("interListenerSet()");

	// on a mouse click, events "mouseup" and "click" happen indivisually.
	// incase listing and handling "mouseup",
	// "click" may happen as another event
	// that is out of "mouseup" handling.
	// and the event "click" might make page move to another page
	// that can not be stoped by "mouseup" handling.
	// so handle "click" at here to prevent to move to href page
	// by event.preventDefault()

	const that = this;
	const ele = this.bxCenter().bxTop().ele();
	const bloxFm = arguments[0];

	const f2 = function(event) {
	    that.onInterReq.apply(that, [event, bloxFm]);
	    // that.editorClose();
	};
	this.interListener(f2);
	
	ele.addEventListener("click", this.interListener());
	
    } // end of class Editor interListenerSet 

    interListenerRemove() {
	// this.log("interListenerRemove()");

	const ele = this.bxCenter().bxTop().ele();
	ele.removeEventListener("click", this.interListener());
	this.interListener(undefined);	
	
    } // end of class Editor interListenerRemove 

    interListener() {
	if(0 < arguments.length){ this.interListenerV = arguments[0]; }
	return this.interListenerV;
    } // end of class Editor interListener 

    // Procedure when a target element was selected.
    onInterReq(event, bloxFm) {
	// this.log2("onInterReq()");

	// stop moving to href when anchor is clicked to select
	event.preventDefault(); // prevent to move to href

	// blox clicked
	const bloxTo = this.bloxFromElePart(event.target);
	// this.log2("onInterReq()", "bloxTo:" + bloxTo.bloxTreeNameKey());

	// clicked this.eleDrawInter(), ignore
	if(bloxTo == this){
	    // this.log2("onInterReq()", "selected self");
	    return;
	}

	if(! this.onInterPrecheck(...arguments, bloxTo)){ return; }
	
	this.interListenerRemove(this.parentBx());	

	if(this.currentStatus().editType == "move"){
	    if(! this.onInterMoveCheck(...arguments, bloxTo)){ return; }
	    bloxFm.editor().editorMove(bloxTo);
	    bloxFm.editor().editorClose(...arguments);
	}

	if(this.currentStatus().editType == "insert"){
	    // do not editorClose because new editor for new item will be open 
	    // bloxFm.editor().editorClose(...arguments);
	    bloxFm.editor().editorInsert(bloxTo);
	}

	// // THIS make editor close in case of insertion
	// bloxFm.editor().editorClose(...arguments);
	
    } // end of class Editor onInterReq 

    onInterPrecheck(event, bloxFm, bloxTo) {
	// this.log2("onInterPrecheck()");
	
	// blox selected, where move to
	if(bloxTo == undefined){

	    this.log2("onInterPrecheck()", "bloxTo:" + bloxTo);
	    
	    // bloxFm.editor().editorClose(...arguments);
	    return;
	}
	
	// the editor was clicked
	if(bloxTo == bloxFm.editor()){
	    
	    this.log2("onInterPrecheck()", "editor was clicked");
	    
	    return; }
	
	// different blox type was clicked
	// ignore and close the editor
	if(bloxFm.constructor.name != bloxTo.constructor.name){

	    this.log2("onInterPrecheck()", "different class was clicked");
	    
	    // bloxFm.editor().editorClose(...arguments);
	    return;
	}

	return true;

    } // end of class Editor onInterPrecheck 

    onInterMoveCheck(event, bloxFm, bloxTo) {
	
	// same
	if(bloxFm == bloxTo){
	    bloxFm.editor().editorClose(...arguments);
	    return;
	}
	
	// can not move to its child
	if(this.movingToMyChild(...arguments)){
	    this.result("err","Can not move to its child!");
	    this.editorMoveOpen();
	    return; 
	}

	return true;
	
    } // end of class Editor onInterMoveCheck 

    // menu to select before / after / child
    eleDrawInter() {
	// this.log2("eleDrawInter()","");

	let html = this.htmlEditorBox;
	html = this.htmlPhReplace(html, this.htmlEditorTargetSelect);
	let ele = this.eleFromHtml(html);

	const eleType = this.querySelectorBx(ele, "editorSelectType");
	eleType.innerHTML = this.currentStatus().editType;
	
	// <span class"{BXPF=editorSelectOption}"></span>
	const eleOp = this.querySelectorBx(ele, "editorSelectOption");
	if(this.currentStatus().editOption == "child"){
	    eleOp.innerHTML = "as child";
	} else {
	    eleOp.innerHTML = this.currentStatus().editOption;
	}

	this.setMoveEvent(ele);
	
	this.ele(ele);
	
    } // end of class Editor eleDrawInter 

    movingToMyChild() {
    } // end of class Editor movingToMyChild 

    menuMoveItem = [
	"ActCancel", "OptionSetBefore", "OptionSetAfter", "OptionSetChild"
    ];

    setMoveEvent() {
	// this.log("setMoveEvent()");
	const ele = arguments[0];
	for(const name of this.menuMoveItem){
	    let fname = "editor"+name;

	    // if not method exits, skip
	    const finst = this.methodInstance(fname);
	    if(finst == undefined){
		// this.log("setMoveEvent() skip:" + fname);
		continue; }

	    const eleSws = this.querySelectorAllBx(ele, fname);
	    // if(eleSws.length == 0){ continue;}
	    const that = this;
	    for(const eleSw of eleSws){
		// not link to finst directory, but via editorMoveClick()
		// that make possible to controll all editor move event
		eleSw.addEventListener('click', function(event){
		    that.editorMoveClick.apply(that, [event, name]);
		} );
	    }

	}
    } // end of class Editor setMoveEvent 

    editorMoveClick() {
	// this.log("editorMoveClick()");

	const event = arguments[0];
	const name = arguments[1];

	event.stopPropagation(); // prevent to call this.move()
	
	const finst = this.methodInstance("editor"+name);
	if(finst == undefined){ return; }
	finst.apply(this, [...arguments]);	
	
    } // end of class Editor editorMoveClick 

    // editorOptionSetChild(event) {
    editorOptionSetChild() {
	// this.log("editorOptionSetChild");
	// this.currentStatus().moveOption = "child";
	this.currentStatus().editOption = "child";
	this.eleDraw();
    } // end of class Editor editorOptionSetChild 

    // editorOptionSetBefore(event) {
    editorOptionSetBefore() {
	// this.log("editorOptionSetBefore");
	// this.currentStatus().moveOption = "before";
	this.currentStatus().editOption = "before";
	this.eleDraw();
    } // end of class Editor editorOptionSetBefore 

    // editorOptionSetAfter(event) {
    editorOptionSetAfter() {
	// this.log("editorOptionSetAfter");
	// this.currentStatus().moveOption = "after";
	this.currentStatus().editOption = "after";
	this.eleDraw();
    } // end of class Editor editorOptionSetAfter

    // editorMove() {} // end of class Editor editorMove

    eledrawinsert() {

	let html = this.htmlEditorBox;
	html = this.htmlPhReplace(html, this.htmlEditorTargetSelect);
	let ele = this.eleFromHtml(html);

	const eleType = this.querySelectorBx(ele, "editorSelectType");
	eleType.innerHTML = this.currentStatus().editType;
	
	const eleOp = this.querySelectorBx(ele, "editorSelectOption");
	eleOp.innerHTML = this.currentStatus().editOption;
	
	this.setInsertEvent(ele);
	
	this.ele(ele);
	
    } // end of class Editor eleDrawInsert 

    editorDelete() {

	this.eleVisibleSet(
	    this.ele(),
	    {
		"htmlEditorEnter" : 0
		, "editorDeleteConfirm" : 1
		,"editorMoveMenu" : 0
	    }
	);
	
    } // end of class Editor editorDelete 
    
} // end of class Editor end 

// class Menu
// class Navi
// class Index : Index of Items
// class Item : A page has topics and each topic is Item

// Menu
// class Menu
// class MenuEditor
class Menu extends Blox {

    elePageTop() {
	if(0 < arguments.length){ this.elePageTopV = arguments[0]; }
	return this.elePageTopV;
    } // end of class Menu elePageTop 

    eleDrawInst() {
	// console.log("wc.js class Menu eleDrawInst()");

	let html = this.htmlMenu;
	let ele = this.eleFromHtml(html);

	this.setEvent(ele);
	
	this.eleVisibleSet(
	    ele,
	    {
		'menuSave' : 0
	    }
	);
	
	this.ele(ele);
	
    } // end of class Menu eleDrawInst 
    
    htmlMenu = (`
    <table class="editModeTable_">
      <tr>
	<td>
	  Edit MODE 

	  <input type="button" value="Exit" class="{BXPF=menuExit}">
	  <input type="button" value="Save" class="{BXPF=menuSave}">


(
	  <input type="button" value="Page Move" class="{BXPF=menuPageMoveReq}">
	  <input type="button" value="href_reference" class="{BXPF=menuhref_reference">
	  <input type="button" value="page_json" class="{BXPF=menupage_json_open">


	  <input type="button" value="Set Group Top" class="{BXPF=menugroup_top_set">
)

	</td>
      </tr>

    </table>
`); // end of class Menu htmlMenu

    menuItem = [
	"menuExit"
	,"menuSave"
	,"menuPageMoveReq"
	// ,""
    ];

    setEvent() {
	const ele = arguments[0];

	for(const fname of this.menuItem){
	    // let fname = "editor"+name;
	    const eleSws = this.querySelectorAllBx(ele, fname);
	    if(eleSws.length == 0){ continue;}
	    const that = this;
	    for(const eleSw of eleSws){
		eleSw.addEventListener('click', function(event){
		    that.menuClick.apply(that, [event, fname]);
		} );
	    }
	}

    } // end of class Menu setEvent 

    menuClick() {
	// this.log("menuClick");

	const event = arguments[0];
	const fname = arguments[1];
	
	const fcode = new Function("return this."+fname+";");
	const finst = fcode.apply(this);
	if(finst == undefined){ return; }
	finst.apply(this, [...arguments]);
	
    } // end of class Menu menuClick 

    menuExit() {
	// this.log("menuExit()");

	const currentBlox = this.currentBlox();
	currentBlox.editor().editorClose();

    } // end of class Menu menuExit

    menuSave() {
	// this.log("menuSave()");

	if(! this.changed()){ return; }
	if(this.currentBlox()){ return; }

	const res = postData("json_save", this.bxCenter().bxTop().data());

	res.then(
	    data => {
		console.log("wc.jp function save res:" + data.res);
		if(data.res == "post_handle page_json_save"){}
		else{
		    alert(err + "\n Try to save again!");
		    return;
		}
		
		this.changed(undefined);
		this.editorClose();
	    }
	)
	    .catch((err) => {
		alert(err + "\n Try to save again!");
	    }
		  )
	;

	// this.changed(undefined);
	// this.editorClose();

    } // end of class Menu menuSave

    menuPageMoveReq() {
	// this.log2("menuPageMoveReq()","");

	this.editor().currentStatus().editType = "pageMove";
	alert("Close the current editor, then Page Momve menu is comming up!");
	
    } // end of class Menu menuPageMoveOpen 

    changed() {
	if(0 < arguments.length){
	    if(arguments[0]){
		this.changedV = true;
		this.swSaveOn();
	    } else {
		this.changedV = false;
	    }
	}
	return this.changedV;
    } // end of class Menu changed

    swSaveOn() {
	this.eleVisibleSet(
	    this.ele(),
	    {
		'menuExit' : 0
		,'menuSave' : 1
	    }
	);
    } // end of class Menu swSaveOn 

    editorOpenListenerSet() {

	// set eventListener to open editor
	const that = this;

	this.elePageTop().addEventListener('contextmenu', function(event) {
	    that.editorOpenListener.apply(that, [event]);
	});
	
	this.hrefListenerAdd();
	
    } // end of class Menu editorOpenListenerSet 

    // this.editorOpenListener is called when right click on any part of the body
    // eventuary editorOpenListener() will be called by each object clicked
    editorOpenListener() {
	
	if(event.button != 2){ return;} // click right;
	
	let blox = this.bloxToOpen(event);
	this.editorOpen(blox);

    } // end of class Menu editorOpenListener 

    bloxToOpen(event) {
	return this.bloxFromElePart(event.target);
    } // end of class Menu bloxToOpen 

    // If Menu.currentBloxV has a value, an editor menu is open.
    currentBlox() {
	if(0 < arguments.length){ this.currentBloxV = arguments[0]; }
	return this.currentBloxV;
    } // end of class Menu currentBlox 

    // this.editorOpen(blox);
    // it calls blox.editor().eleDrawInst()
    editorOpen(blox) {
	// this.log2("editorOpen()");

	if(blox.editor() == undefined){ return; }
	
	if(this.currentBlox()){

	    // if any editor is open, it must be saved or closed
	    // on editor's will
	    // otherwise it might discard editing data
	    return;
	    
	    // opening the current blox again,
	    // ignoer the request
	    if(this.currentBlox() == blox){ return; }

	    // this is not right way, result will not changed before enter
	    // // otherwise editing is discarded
	    // if(this.currentBlox().editor().result("changed")){ return; }
	    
	    this.currentBlox().editor().editorClose();
	}

	// draw menu
	// before blox.editor().eleDraw();
	// because class MenuEditor.eleDrawInst() requires this.eleDraw()
	if(! this.ele()){ this.eleDraw(); }

	blox.editor().eleDraw();

	this.currentBlox(blox);

	// call this.menuVisibleSet() after blox.editor().eleDraw()
	this.menuVisibleSet();
	
	// when target was selected by a click,
	// class Menu hrefEventHandle makes it move to the href target
	// so remove hrefEventHandle
	// on this.editorOpen(),
	// this.menu().hrefListenerAdd() might be called
	// if this.currentBlox() exists
	// so this.hrefListenerRemove() should be called
	// after this.editorOpen()
	this.hrefListenerRemove();
	
    } // end of class Menu editorOpen 

    editorClose() {

	// const currentBlox = this.currentBlox();
	// if(currentBlox == undefined){ return; }

	// if it is blank item, delete not only the editor drawing
	// but also currentBlox().ele() drawing where editor drawing on
	// if(this.currentBlox().parentBx().isBlank()){
	// if(this.currentBlox().isBlank()){
	if(this.currentBlox() && this.currentBlox().isBlank()){
	    this.currentBlox().ele(undefined);
	}

	this.currentBlox(undefined);

	if(this.editor().currentStatus().editType == "pageMove"){
	    this.editorOpen(this);
	    return;
	}
	
	this.hrefListenerAdd();
	
	this.menuVisibleSet();
	
    } // end of class Menu editorClose 

    menuVisibleSet() {

	if(this.currentBlox()){
	    this.eleVisibleSet(this.ele(), { 'menuExit' : 1
					     ,'menuSave' : 0 });
	} else {
	    if(this.changed()){
		this.eleVisibleSet(this.ele(), { 'menuExit' : 0
						 ,'menuSave' : 1 });
	    } else {
		this.ele(undefined); 
	    }
	}

    } // end of class Menu menuVisibleSet 

    hrefEventHandle(event) {
	// this.log("hrefEventHandle()");

	// prevent to move to href
	// espacialy avoid to move to another page without saveing editing
	event.preventDefault();

	let href = event.target.getAttribute("href");

	// href : #abc
	// move to #abc .
	// #: move to top
	if(href == "#"){
	    window.scrollTo(0, 0);
	    return;
	}
    
	if(href.match(/^#(.+)/)){
	    location.href = href;
	    // remove #
	    // scrollHash(href.slice(1));
	    return;
	}

	if(href == "javascript:history.back()"){
	    // console.log("wc.js class Menu hrefEventHandle() href:" + href);
	    // alert(href);
	    javascript:history.back();
	    return;
	}

	if(this.changed()){
	    alert("Save or discard changes before move page!");
	    return;
	}

	let data = {"href" : href};
    
	console.log("wc.js class Menu hrefEventHandle() post href:" + href);
	let res = postData("href", data);
	
	res.then(data => {
	    // alert("wc.jp class Menu hrefEventHandle");
	    if(data.dest){
		location.href = data.dest;
	    }
	
	});

	event.preventDefault(); // prevent to move to href

    } // end of class Menu hrefEventHandle 
    
    hrefEventListener() {
	if(0 < arguments.length){ this.hrefEventListenerV = arguments[0]; }
	return this.hrefEventListenerV;
    } // end of class Menu hrefEventListener 
    
    hrefListenerAdd() {
	// this.log("hrefListenerAdd()");

	// already set
	if(this.hrefListenerStatus()){ return; }

	const that = this;
	let eles = this.elePageTop().getElementsByTagName("a");

	const f = function(event){
	    that.hrefEventHandle.apply(that, [event]);
	};
	this.hrefEventListener(f);
	
	for(let ele of eles){
	    let href = ele.getAttribute("href");
	    if(href){
		ele.addEventListener("click", this.hrefEventListener());
	    }
	}

	this.hrefListenerStatus(true); // set	
	
    } // end of class Menu hrefListenerAdd 

    hrefListenerRemove() {
	// this.log("hrefListenerRemove()");

	const that = this;
	let eles = this.elePageTop().getElementsByTagName("a");
	for(let ele of eles){
	    let href = ele.getAttribute("href");

	    if(href){
		ele.removeEventListener("click", this.hrefEventListener());
	    }
	}
	
	this.hrefListenerStatus(undefined); // unset
	
    } // end of class Menu hrefListenerRemove

    // status = this.hrefListenerStatus(); // get
    // this.hrefListenerStatus(true); // set
    // this.hrefListenerStatus(undefined); // unset
    hrefListenerStatus() {
	if(0 < arguments.length){ this.hrefListenerStatusV = arguments[0];}
	return this.hrefListenerStatusV;
    } // end of  class Menu hrefListenerStatus

    eleTargetChild() {
	// this.log2("eleTargetChild()", "this.ele(): " + this.ele());
	return this.ele();
    } // end of class Menu eleTargetChild 
    
} // end of class Menu end

// this is for page move indeed
class MenuEditor extends Editor {

    eleDrawInst() {
	// this.log2("eleDrawInst()", "");

	if(this.currentStatus().editType == "pageMove"){
	    return this.eleDrawInstPageMove();
	}
	
    } // end of class MenuEditor eleDrawInst()

    eleDrawInstPageMove() {

	let html = this.htmlEditorBox;
	html = this.htmlPhReplace(html, this.htmlEditorURL);
	html = this.htmlPhReplace(html, this.htmlEditorEnter);
	
	let ele = this.eleFromHtml(html);
	
	this.eleVisibleSet(ele, {"editorNewPage" : 0});
	
	this.ele(ele);
	
    } // end of eleDrawInstPageMove

    htmlEditorURL = (`
      <tr>
      <td>Parent URL</td>
      <td><input class="{BXPF=parentUrl}"></td>	
      </tr>
      <tr>
      <td>Destination URL</td>
	<td><input class="{BXPF=destUrl}"></td>
      </tr>
      <tr>
      <td></td>
	<td></td>
      </tr>
	<!--placeHolder-->
`); // end of class MenuEditor htmlEditorURL

    editorClose() {
	delete this.currentStatus().editType;
	super.editorClose();
    } // end of editorClose
    
    editorEnter() {
	// this.log2("editorEnter()","");

	if(this.currentStatus().editType == "pageMove"){
	    return this.editorEnterPageMove();
	}
	
    } // end of class MenuEditor editorEnter

    editorEnterPageMove() {
	const parentUrlEle = this.querySelectorBx(this.ele(), "parentUrl");
	const parentUrl = parentUrlEle.value;
	if(parentUrl.length == 0){
	    this.result("err","parent URL is emply!");
	}	
	
	const destUrlEle = this.querySelectorBx(this.ele(), "destUrl");
	const destUrl = destUrlEle.value;
	if(destUrl.length == 0){
	    this.result("err","Destination URL is emply!");
	}
	
	if(0 < this.result().err.length){
	    this.eleDraw();
	    return;
	}
	
	let data = {"parent_url" : parentUrl, "dest_url" : destUrl};
	let res = postData("page_move", data);

	delete this.currentStatus().editType;

	// super: class Editor this class extends on.
	super.editorEnter();
	
    } // end of class MenuEditor editorEnterPageMove 
    
} // end of class MenuEditor end  

// Navi
// class Navi
// class NaviItem
// class NaviItemEditor
// 
class Navi extends Blox {

    dataParent() {
	if(0 < arguments.length){ this.dataParentV = arguments[0]; }
	return this.dataParentV;
    } // end of class Navi dataParent 

    data() {
	return this.dataParent().navi;
    } // end of class Navi data 
    
    dataChild(child) {

	if(child.constructor.name == "NaviItem"){
	    return this.data()[child.key()];
	}
	
    } // end of class Navi dataChild 

    eleDrawInst() {
	// this.log("eleDrawInst()");
	
	const ele = document.createElement("div");

	// naviItem target
	const eleItemTarget = document.createElement("div");
	const itemTargetName = this.bloxPrefix("itemTarget");
	eleItemTarget.classList.add(itemTargetName);
	ele.appendChild(eleItemTarget);

	// naviItemEditor target
	const eleEditorTarget = document.createElement("div");
	const editorTargetName = this.bloxPrefix("itemEditorTarget");
	eleEditorTarget.classList.add(editorTargetName);
	ele.appendChild(eleEditorTarget);
		
	this.ele(ele);

	this.naviDraw();
	
    } // end of class Navi eleDrawInst 

    eleTargetChild() {
	// this.log("eleTargetChild()");

	return this.querySelectorBx(this.ele(), "itemTarget");
	
    } // end of class Navi eleTargetChild

    eleEditorTarget() {
	return this.querySelectorBx(this.ele(), "itemEditorTarget");
    } // end of class Navi eleEditorTarget 

    naviDraw() {
	// this.log("naviDraw");

	const data = this.data();
	for(let i=0; i<data.length; i++ ){
	    const naviItem = this.bloxChild("NaviItem", i, {"create" : true});
	    naviItem.eleDraw();
	}
	
    } // end of class Navi naviDraw

    naviItemDelete(naviItem) {

	naviItem.clear();

	this.childRemoveSibling(naviItem);
	
	// delete data
	this.data().splice(naviItem.key(), 1);

    } // end of class Navi naviItemDelete 

    naviItemBlank() {
	// this.log("naviItemBlank()");

	const naviItem =
	      this.bloxChild("NaviItem", this.idBlank, {"create" : true});
	naviItem.navi(this);
	return naviItem;

    } // end of class Navi naviItemBlank 

} // end of class Navi end 

class NaviItem extends Blox {

    navi() {
	return this.parentBx();
    } // end of class NaviItem navi 

    targetNext() {
	// if(0 < arguments.length){ this.targetNextV = arguments[0]; }
	// return this.targetNextV;

	// only case naviItemBlank
	if(! this.isBlank()){ return; }
	const bloxTo = this.currentStatus().bloxTo;
	
	if(this.currentStatus().editOption == "before"){
	    return bloxTo.ele();
	}
	
	if(this.currentStatus().editOption == "after"){
	    const naviItemNext = bloxTo.naviItemNext();
	    if(naviItemNext){
		return naviItemNext.ele();
	    }
	    
	}
	
    } // end of class NaviItem targetNext 
    
    eleDrawInst() {
	// this.log("eleDrawInst()");

	if(this.isBlank()){
	    return this.eleDrawInstBlank();
	}	

	let ele = document.createElement("span");
	ele.appendChild(this.eleAnchor());
	
	// not the last
	// this.key() 0, 1, ... , this.idBlank
	if(this.key() + 1 != this.navi().data().length){
	    ele.appendChild(this.eleDelimiter());
	}

	this.ele(ele);
	
    } // end of class NaviItem eleDrawInst

    eleAnchor() {
	// this.log("eleAnchor()");

	let ele = document.createElement("span");
	let eleA;
	const href = this.data()[1];
	if(href){
	    eleA = document.createElement("a");
	    eleA.setAttribute("href", href);
	} else  {
	    eleA = document.createElement("span");
	}
	const name = this.data()[0];
	eleA.appendChild(document.createTextNode(name));
	ele.appendChild(eleA);

	return ele;
	
    } // end of eleAnchor

    eleDelimiter() {
	return document.createTextNode(" > ");
    } // end of eleDelimiter

    eleDrawInstBlank() {

	let ele = document.createElement("span");

	// check if case inserting to after the last NaviItem
	let theLast;
	if(
	    // target is the last
	    this.currentStatus().bloxTo.key() == this.navi().data().length -1
		&&
		this.currentStatus().editOption == "after"
	){
	    // this.log("eleDrawInst() at the last");
	    theLast = true;
	}

	// ... (the last NaviItem) ">" new
	if(theLast){
	    ele.appendChild(this.eleDelimiter());
	}
	
	ele.appendChild(this.eleAnchor());

	// ... new ">" ...
	if(!theLast){
	    ele.appendChild(this.eleDelimiter());
	}

	this.ele(ele);
	
    } // end of class NaviItem eleDrawInstBlank 

    eleTargetChild() {
	return this.eleTarget();
    } // end of class NaviItem eleTargetChild 

    // this.move(naviItemTo, option);
    // move data
    move(naviItemTo, option) {
	// this.log("move()");

	const data = this.navi().data();

	const data2 = [];
	for(let i=0; i<data.length; i++){
	    if(i == this.key()){ continue; }
	    if(i == naviItemTo.key()){
		if(option.before){
		    data2.push(this.data());
		}
		data2.push(data[i]);
		if(option.after){
		    data2.push(this.data());
		}
	    } else {
		data2.push(data[i]);
	    }
	}
	
	this.navi().dataParent().navi = data2;
	
    } // end of class NaviItem move

    itemDelete() {
	this.navi().naviItemDelete(this);
    } // end of class NaviItem itemDelete

    naviItemNext() {
	// this.log2("naviItemNext", "");
	
	const keyNext = this.key() + 1;
	
	const naviItemNext = this.navi().bloxChild("NaviItem", keyNext);
	return naviItemNext;

    } // end of class NaviItem naviItemNext

} // end of class NaviItem end 

class NaviItemEditor extends Editor {

    naviItem() {
	return this.parentBx();
    } // end of class NaviItemEditor naviItem 

    eleTarget() {
	return this.naviItem().navi().eleEditorTarget();
    } // end of class NaviItemEditor eleTarget 

    eleDrawInst() {
	// this.log("eleDrawInst()");

	if(this.currentStatus().editorInter){
	    return this.eleDrawInter();
	}
	
	let html = this.htmlEditorBox;
	html = this.htmlPhReplace(html, this.htmlEditorTitleHref);
	html = this.htmlPhReplace(html, this.htmlEditorEnter);
	html = this.htmlPhReplace(html, this.htmlEditorInter);

	let ele = this.eleFromHtml(html);

	this.dataSet(ele);

	if(this.naviItem().isBlank()){
	    this.eleVisibleSet(
		ele,
		{ "editorMoveMenu" : 0
		  , "editorEnter" : 1
		}
	    );
	}
	
	this.eleVisibleSet(ele, {"editorNewPage" : 0});
	
	this.ele(ele);

    } // end of class NaviItemEditor eleDrawInst 

    names = ["title", "href"];
    
    dataSet(ele) {

	// const naviItem = this.parentBx();
	// title: naviItem.data()[0]
	//  href: naviItem.data()[1]

	// const names = ["title", "href"];
	for(let i=0; i<this.names.length; i++){
	    const eleName = "input"+firstUpper(this.names[i]);
	    const eleTgt = this.querySelectorBx(ele, eleName);
	    let value = this.naviItem().data()[i];
	    if(value == undefined){
		if(name == "title"){ value = ""; }
		if(name == "href"){ value = "#"; }
	    }
	    eleTgt.value = value;
	}
	
    } // end of class NaviItemEditor dataSet

    editorEnter() {
	// this.log("editorEnter()");

	const naviItem = this.parentBx();

	for(let i=0; i<this.names.length; i++){
	    const eleName = "input"+firstUpper(this.names[i]);
	    const eleTgt = this.querySelectorBx(this.ele(), eleName);
	    if(this.names[i] == "title"){
		if(eleTgt.value.length == 0){
		    this.result("err", "no title");
		    continue;
		}
	    }
	    if(this.naviItem().data()[i] != eleTgt.value){
		this.naviItem().data()[i] = eleTgt.value;
		this.result("changed", true);
	    }
	}
	    
	if(this.isBlank()){ return this.editorEnterNew(); }
	
	super.editorEnter();

	// remove naviItem and delimiter
	// and draw new naviItem
	this.naviItem().navi().eleDraw()	

    } // end of class NaviItemEditor editorEnter 

    eleDrawInter() {

	super.eleDrawInter();

	// no child
	// before super.eleDrawInter(), this.ele() does not exist
	this.eleVisibleSet(this.ele(), {'editorOptionSetChild' : 0});
	
    } // end of class NaviItemEditor eleDrawInter

    // this.editorMove();
    editorMove() {
	// this.log("editorMove()");
	
	const naviItemTo = arguments[0];

	const moveOption = {};
	// "after" / "before" / "child"
	moveOption[this.currentStatus().editOption] = true;

	const naviItemFm = this.parentBx();
	naviItemFm.move(naviItemTo, moveOption);

	// remove naviItem and delimiter
	// and draw new naviItem
	this.naviItem().navi().childRemoveSibling(this.naviItem());
 	this.naviItem().navi().eleDraw()	
	
    } // end of class NaviItemEditor editorMove 
    
    // this.editorInsert(bloxTo);
    // bloxTo: NaviItem selected
    editorInsert(bloxTo) {
	// this.log("editorInsert()");

	const navi = this.naviItem().navi();
	const naviItemBlank = navi.naviItemBlank();
	
	// "before" / "after"
	const option = this.currentStatus().editOption
	naviItemBlank.currentStatus().editOption = option;
	naviItemBlank.currentStatus().bloxTo = bloxTo;

	// close current editor to let open editor of naviItemBlank
	this.editorClose();

	naviItemBlank.data(["new", ""]);
	naviItemBlank.eleDraw();
	this.menu().editorOpen(naviItemBlank);
	
    } // end of class NaviItemEditor editorInsert

    editorEnterNew() {
	// this.log2("editorEnterNew()");

	if(0 < this.result().err.length){
	    super.editorEnter();
	    return;
	}

	const data = this.naviItem().navi().data();
	const data2 = [];
	const bloxTo = this.naviItem().currentStatus().bloxTo;
	const option = this.naviItem().currentStatus().editOption;
	const dataNew = [
	    // title, href
	    this.naviItem().data()[0], this.naviItem().data()[1]
	];

	for(let i = 0; i<data.length; i++){
	    if(i == bloxTo.key()){
		if(option == "before"){ data2.push(dataNew); }
		data2.push(data[i]);
		if(option == "after"){ data2.push(dataNew); }
	    } else {
		data2.push(data[i]);
	    }
	}
	
	const dataParent = this.naviItem().navi().dataParent();
	dataParent.navi = data2;

	this.result("changed", true);
	this.editorClose();
	
	// remove naviItem and delimiter
	// and draw new naviItem
	this.naviItem().navi().childRemoveSibling(this.naviItem());
 	this.naviItem().navi().eleDraw()	
	
    } // end of class NaviItemEditor editorEnterNew 

    editorDeleteExecute() {
	// this.log("editorDeleteExecute()");

	this.naviItem().itemDelete();

	// draw all naviItems with new data order
	this.naviItem().navi().eleDraw();

	this.result("changed", true);
	// call this.editorClose() after set result to apply the changes
	this.editorClose();

    } // end of class NaviItemEditor editorDeleteExecute
    
} // end of class NaviItemEditor end 

class ItemsCenter extends Blox {

    dataItem() {
	const item = arguments[0];
	if(item == undefined){ return; }
	if(item.constructor.name != "Item"){ return; }
	return this.data().data[item.key()];
    } // end of class ItemsCenter dataItem 

    // return boolean if href in argument is alreaky used 
    hrefInUseLocal() {
	const hrefArg = arguments[0];

	if(hrefArg == undefined){ return; }

	// does not start with #
	if(! hrefArg.match(/^#(.+)/)){ return false; }

	for(let key of Object.keys(this.data().data)){
	    if(this.data().data[key].href == hrefArg){
		return true;
	    }
	}

	return false;

    } // end of class ItemsCenter hrefInUseLocal 

    //	let id_data = this.page_json["data"]["subsection"]["id"];
    idNew() {
	// this.log2("idNew()", "");

	const idData = this.data().id;

	this.bxCenter().bxTop().menu().changed(true);
	
	if(0 < idData.id_notinuse.length){
	    return idData.id_notinuse.shift();
	}

	const idNew = idData.id_next;
	idData.id_next = idNew + 1;

	return idNew;
	
    } // end of class ItemsCenter idNew 

    // idNew = this.itemNew(itemParent);
    // make an item on this.data().data
    // since it create a data for new item,
    // but no instance yet, so return its id
    itemNew() {
	// this.log2("itemNew()", "");
	
	const itemParent = arguments[0];
	
	const idNew = this.idNew();

	// this.log2("itemNew()", "idNew:" + idNew);

	// create data
	const data = { "child" : [], "content" : []};
	if(itemParent){
	    data.parent = itemParent.key();
	}
	this.data().data[idNew] =  data;

	return idNew;
	
    } // end of class ItemsCenter itemNew 

    idDiscard() {
	const idDiscarded = arguments[0];
	if(idDiscarded == undefined){ return; }

	this.data().id.id_notinuse.push(idDiscarded);
	this.bxCenter().bxTop().menu().changed(true);
	
    } // end of class ItemsCenter idDiscard 

    itemDelete(item) {
	// this.log("itemDelete()");

	item.ele(undefined);
	
	const thisKey = item.key();
	const itemParent = item.itemParent();

	delete this.data().data[item.key()];

	const parentChild = itemParent.data().child;
	for(let i=0; i<parentChild.length; i++){
	    if(parentChild[i] == thisKey){
		parentChild.splice(i, 1);
		break;
	    }
	}
	
    } // end of class ItemsCenter itemDelete

    itemChildDelete(item) {
	// this.log("itemChildDelete()");

	const child = item.data().child;
	for(let i=0; i<child.length; i++){
	    const key = child[i];
	    const itemChild = item.child(key);
	    
	    this.itemChildDelete(itemChild);
	    this.itemDelete(itemChild);
	}

    } // end of class ItemsCenter itemChildDelete 

    itemAndChildDelete(item) {
	// this.log("itemAndChildDelete()");
	this.itemChildDelete(item);
	this.itemDelete(item);
    } // end of class ItemsCenter itemAndChildDelete 

} // end of class ItemsCenter end 

// class Item
// class Contents : Contents is contents of Item
//
// class Item: a topic of the page.
// Item have class Contents.
// Item can have some Items as its children.
// Each Item have a class Index for the page's index list.
class Item extends Blox {

    itemsCenter() {
	if(0 < arguments.length){
	    this.itemsCenterV = arguments[0];
	}
	return this.itemsCenterV;
    } // end of class Item itemsCenter 

    data() {
	if(this.isBlank()){ return this.dataBlank(...arguments); }
	return this.itemsCenter().dataItem(this);
    } // end of class Item data 
    
    itemParent() {
	// itemBlank need to be given itemParent
	const parent2 = this.parentBx();
	if(parent2.constructor.name == this.constructor.name){ return parent2; }
    } // end of class Item itemParent 

    eleDrawInst() {
	// this.log("eleDrawInst()");

	// indexItem draw before index draw
	// because index of this is to be drawn under indexItem of this
	this.indexItem().eleDraw();

	if(this.isBlank()){ return; }

	// draw index before draw child items
	this.index().eleDraw();

	this.contents().eleDraw();
	
    } // end of class Item eleDrawInst

    itemNext() {
	// this.log("itemNext()");

	// this.itemNextV is used only for Item Blank
	if(this.isBlank()){
	    if(0 < arguments.length){ this.itemNextV = arguments[0]; }
	    if(this.itemNextV){ return this.itemNextV; }
	}

	const itemParent = this.itemParent();
	if(itemParent == undefined){ return undefined; }

	const siblingList = itemParent.children();
	let founded;
	for(let sibling of siblingList){

	    // if this is moved,
	    // item of this.itemParent().children() with this.key()
	    // might be deferent instance from this
	    // so compare its key value, not instance
	    // if(sibling == this){}
	    if(sibling.key() == this.key()){
		founded = true;
		continue;
	    }
	    // next to this
	    if(founded){
		return sibling;
	    }
	}

	return undefined;
	
    } // end of class Item itemNext 

    eleTargetChild() {
	// this.log("eleTargetChild()");

	const child = arguments[0];
	if(child == undefined){ return; }

	// IndexItem
	if(child.constructor.name == "IndexItem"){
	    const itemParent = this.itemParent();

	    if(! itemParent){ return; }
	    const index = itemParent.index();

	    if(index == undefined){ return; }
	    
	    return index.ele();
	}
	
	// Contents
	// at first, top of items tree
	// eleTarget is set with this.item().contents().eleTargetName()
	// and the other items take save element as a target
	if(child.constructor.name == "Contents"){
	    const itemParent = this.itemParent();
	    if(itemParent == undefined){ return; }
	    return itemParent.contents().eleTarget();
	}
	
    } // end of class Item eleTargetChild 

    // item = this.child(key);
    child() {
	// this.log("child()");
	
	const key = arguments[0];
	if(key == undefined){ return; }

	const item = this.bloxChild("Item", key, {"create" : true});
	item.itemsCenter(this.itemsCenter());
	return item;
	
    } // end of class Item child 

    children() {
	// this.log2("children()","");
	// this.log("children()");
	
	const children = [];

	if(this.data().child == undefined){ return children; }
	
	for(let key of this.data().child){
	    children.push(this.child(key));
	}
	
	return children;
	
    } // end of class Item children

    // itemChild = this.childNew({"target": itemTarget, "before" : true});
    childNew() {
	// this.log2("childNew()");
	
	const itemParent = this;
	const idNew = this.itemsCenter().itemNew(itemParent);
	const child = itemParent.data().child;
	
	const option = arguments[0];
	if(option){
	    const itemTarget = option.target;
	    for(let i=0; i<child.length; i++){
		if(child[i] == itemTarget.key()){
		    if(option.before){
			child.splice(i, 0, idNew);
		    } else {
			child.splice(i+1, 0, idNew);
		    }
		    break;
		}
	    }
	} else {
	    child.push(idNew);
	}
	
	const itemChild = this.child(idNew);
	return itemChild;

    } // end of class Item childNew

    hrefStartsSharp() {
	// console.log("wc.js class Item hrefStartsSharp()");
	// gets null if does not match
	if(this.data().href){
	    return(this.data().href.match(/^#(.+)/) != null);
	}
    } // end of class Item hrefStartsSharp

    index() {
	return this.bloxChild("Index", "0", {"create" : true});
    } // end of class Item index 

    indexItem() {
	return this.bloxChild("IndexItem", "0", {"create" : true});
    } // end of class Item indexItem 

    contents() {
	return this.bloxChild("Contents", "0", {"create" : true});
    } // end of class Item contents

    // this.move(itemTo, moveOption);
    // moveOption: {"child": true}, {"before": true}, {"after": true}
    move(itemTo) {
	// this.log("move()");

	const option = arguments[1];

	this.removeFromParent();

	if(option.child){
	    this.putAsChild(itemTo);
	}
	else {
	    this.putBySibling(...arguments);
	}

	super.move(...arguments);
	
    } // end of class Item move 

    removeFromParent() {
	// this.log("removeFromParent()");

	const itemParent = this.itemParent();
	if(! itemParent){ return; }
	
	const childFm = itemParent.data().child;
	for(let i=0; i< childFm.length; i++){
	    if(childFm[i] == this.key()){
		// remove
		childFm.splice(i, 1);
		break;
	    }
	}
	
    } // end of class Item removeFromParent 
    
    putAsChild(itemTo) {
	itemTo.data().child.push(this.key());
	this.data().parent = itemTo.key();
    } // end of class Item putAsChild 

    putBySibling(itemTo) {
    
	const option = arguments[1];
	// let option = arguments[1];
	// if(option == undefined){ option = {}; }

	const itemToParent = itemTo.itemParent();
	const child = itemToParent.data().child;
	let puted;
	for(let i=0; i<child.length; i++){
	    if(child[i] == itemTo.key()){
		if(option.before){
		    child.splice(i, 0, this.key());
		}
		// option.after
		else {
		    child.splice(i+1, 0, this.key());
		}
		puted = true;
		break; 
	    }
	}
	
	if(puted){
	    this.data().parent = itemToParent.key();
	}
	
    } // end of class Item putBySibling 
    
} // end of class Item end 

// class Index
// class IndexItem
// class IndexItemEditor
//
class Index extends Blox {

    item() {
	const parentBx = this.parentBx();
	if(parentBx.constructor.name == "Item"){ return parentBx; }
	
	if(parentBx.item){ return parentBx.item(); }
	
	if(0 < arguments.length){ this.itemV = arguments[0];}
	return this.itemV;
    } // end of class Index item 

    eleTarget() {
 	// console.log("wc.js class Index eleTarget()");
    
	// Index is to hold its children
	// So Index becomes under its IndexItem
	// then Index's children becomes under the Index as IndexItem
	const indexItem = this.item().indexItem();
	// const indexItem = this.parentBx().indexItem();
	if(indexItem){
	    const ele = indexItem.ele();
	    if(ele){ return ele; }
	}
	
	// top of Index may be given a target
	// to return the target given, return super.eleTarget()
	return super.eleTarget();
	
    } // end of class Index eleTarget 

    // this.eleDraw()
    // this.eleDrawInst()
    // this.eleDraw({"drawWithoutChild": true})
    eleDrawInst() {
	// this.log("eleDrawInst()");

	let option = {};
	if(arguments[0]){ option = arguments[0]; }

	const children = this.item().children();
	if(children.length == 0 && !option.drawWithoutChild){ return; }

	const ele = document.createElement("ul");
	this.ele(ele);

    } // end of class Index eleDrawInst 

} // end of class Index end 

class IndexItem extends Blox {

    item() {
	const parentBx = this.parentBx();
	if(parentBx.constructor.name == "Item"){ return parentBx; }
	
	if(parentBx.item){ return parentBx.item(); }
	
	if(0 < arguments.length){ this.itemV = arguments[0];}
	return this.itemV;
    } // end of class IndexItem item 

    eleDrawInst() {
	// this.log2("eleDrawInst()");
	
	const ele = document.createElement("li");

	// a
	const eleA = document.createElement("a");
	eleA.setAttribute("href", this.item().data().href);

	// title
	eleA.appendChild(document.createTextNode(this.item().data().title));
	
	// eleA.appendChild(document.createTextNode(" " + this.item().data().parent +":" + this.item().key()));

	ele.appendChild(eleA);

	// editor target
	const eleEditor = document.createElement("div");
	const keyEditor = this.editor().eleTargetName();
	eleEditor.classList.add(keyEditor);
	ele.appendChild(eleEditor);

	this.ele(ele);
	
    } // end of class IndexItem eleDrawInst 
    
    targetNext() {
	// this.log("targetNext()");

	const itemNext = this.item().itemNext();
	if(itemNext == undefined){ return; }

	const indexItemNext = itemNext.indexItem();
	if(indexItemNext){
	    return indexItemNext.ele();
	}

    } // end of class IndexItem targetNext 
    
} // end of class IndexItem end 

class IndexItemEditor extends Editor {

    item() {
	const parentBx = this.parentBx();
	if(parentBx.constructor.name == "Item"){ return parentBx; }
	
	if(parentBx.item){ return parentBx.item(); }
	
	if(0 < arguments.length){ this.itemV = arguments[0];}
	return this.itemV;
    } // end of class IndexItemEditor item 

    eleDrawInst() {
	// this.log2("eleDrawInst()", "");
	
	// const indexItem = this.parentBx();

	if(this.currentStatus().editorInter){
	    return this.eleDrawInter();
	}
	
	let html = this.htmlEditorBox;
	html = this.htmlPhReplace(html, this.htmlEditorTitleHref);
	html = this.htmlPhReplace(html, this.htmlEditorEnter);
	html = this.htmlPhReplace(html, this.htmlEditorInter);

	let ele = this.eleFromHtml(html);
	
	if(this.item().isBlank()){
	    this.eleVisibleSet(
		ele,
		{ "editorMoveMenu" : 0
		  , "editorNewPage" : 0
		  , "editorEnter" : 1
		}
	    );
	}
	
	this.dataSet(ele);
	
	this.ele(ele);

    } // end of class IndexItemEditor eleDrawInst 
    
    dataSet(ele) {

	for(const name of ["title", "href"]){
	    const eleName = "input"+firstUpper(name);
	    const eleTgt = this.querySelectorBx(ele, eleName);

	    let value = this.item().data()[name];
	    if(value == undefined){
		if(name == "title"){ value = ""; }
		if(name == "href"){ value = "#"; }
	    }
	    eleTgt.value = value;
	}
	
    } // end of class IndexItemEditor dataSet 

    // Handle enter action of the editor.
    editorEnter() {
	// this.log2("editorEnter()");
	
	this.editorEnterTitle();	

	this.editorEnterHref();

	if(this.isBlank()){ return this.editorEnterNew(); }

	super.editorEnter();

 	this.item().eleDraw();
	
    } // end of class IndexItemEditor editorEnter

    editorEnterTitle() {
	// this.log2("editorEnterTitle()");

	const titleEle = this.querySelectorBx(this.ele(), "inputTitle");
	const titleNew = titleEle.value;
	
	if(titleNew.length == 0){
	    this.result("err", "no title");
	} else {
	    if(this.item().data()["title"] != titleNew){
		this.item().data()["title"] = titleNew;
		this.result("changed", true);
	    }
	}
	
    } // end of class IndexItemEditor editorEnterTitle 

    // Handle 
    editorEnterHref() {
	// console.log("wc.js class IndexItemEditor editorEnterHref()");

	// href
	//
	// this.data()["title"] should have some data, no empty
	// 
	// undef to #subtitle0
	// undef to ./abc.html
	// 
	// #subtitle0 to #subtitle0 // no change
	// #subtitle0 to #subtitle1
	// #subtitle0 to abc.html // not allow, must use delete
	// #subtitle0 to undefined // not allow, must use delete
	// abc.html to abc.html // no change
	// abc.html to xyz.html
	// abc.html to #subtitle0 // not allow, mut use delete
	// abc.html to undefined // not allow, mut use delete

	// const result = {};

	const hrefCurrent = this.item().data().href;
	const eleHref = this.querySelectorBx(this.ele(), "inputHref");
	const hrefNew = eleHref.value;
	if(hrefNew == hrefCurrent){
	    // if no change no need to warn
	    // this.result("warning", "href no change");
	    return;
	}

	if(hrefNew == "" || hrefNew == "#"){
	    this.result("err","href no value");
	    return;
	}

	// Do not change outer link to local link
	// hrefCurrent has a value and does not start with #, it it outerlink
	if (hrefCurrent) {
	    // Not start with #, it is outer link.
	    if(! hrefCurrent.match(/^#(.+)/)) {
		// hrefNew starts with #, it is inner link,
		if(hrefNew.match(/^#/)){
		    this.result("err", "Can not change outer link to local link. Delete the link at first.");
		    return;
		}
	    }
	}

	// The href already in use.
	const itemsCenter = this.item().itemsCenter();
	const hrefInuse = itemsCenter.hrefInUseLocal(hrefNew);
	if(hrefInuse){
	    this.result("err", "href alredy in use");
	    return;
	}
	
	this.item().data().href = hrefNew;
	this.result("changed", true);
	
    } // end of class IndexItemEditor editorEnterHref 

    // this.editorMove(bloxTo);
    editorMove() {

	const bloxTo = arguments[0];
	const itemTo = bloxTo.item();
	const itemFm = this.item();

	// moveOption
	const moveOption = {};
	// "after" / "before" / "child"
	moveOption[this.currentStatus().editOption] = true;

	// memorize whether itemTo has children before move
	let childZero;
	if(moveOption.child){
	    if(itemTo.children().length == 0){ childZero = true; }
	}
	
	// set move on item data
	itemFm.move(itemTo, moveOption);
	// super.editorMove(itemTo, moveOption);
	this.result("changed", true);

	// itemTo did not have child, it means no itemTo.index() drawn
	// to draw children , itemTo.index().ele() is required
	// to draw itemTo.index().ele(), it shoul have a child at lease
	// so do itemTo.index().eleDraw() after put a child
	if(childZero){ itemTo.index().eleDraw(); }

	// wondering if beter to itemFm.deleteThis() or leave it

	// const itemFmNew = itemFm.itemsCenter().item(itemFm.key());
	// itemFmNew.eleDraw();

	itemFm.eleDraw();
	
    } // end of class IndexItemEditor editorMove 

    // this.movingToMyChild(event, bloxFm);
    movingToMyChild() {

	// bloxTo
	const event = arguments[0];
	const bloxFm = arguments[1];
	const bloxTo = this.bloxFromElePart(event.target);
	
	// this.log("movingToMyChild()");

	// can not move to its child
	// if bloxFm is a branch of some
	// it does no match with bloxFmParent
	// in such case should compare with bloxFm.parentBx()
	let bloxToParent = bloxTo;
	// bloxFm is editor, get editor's parent
	const bloxFmParent = bloxFm.parentBx();
	while(bloxToParent){
	    if(
		bloxFm == bloxToParent
		    ||
		bloxFmParent == bloxToParent
	      ){
		return true;
	    }
	    bloxToParent = bloxToParent.parentBx();
	}

    } // end of class IndexItemEditor movingToMyChild 
    
    editorDeleteExecute() {
	// this.log("editorDeleteExecute()");

	this.item().itemsCenter().itemAndChildDelete(this.item());

	this.result("changed", true);
	// call this.editorClose() after set result to apply the changes
	this.editorClose();
	
    } // end of class IndexItemEditor editorDeleteExecute 

    // bloxTo: target insert to
    // this.editorInsert(bloxTo);
    editorInsert() {
	// this.log("editorInsert()");
	
	const bloxTo = arguments[0];
	const itemTo = bloxTo.item();
	// const itemFm = this.item();

	// itemParent
	let itemParent;
	if(this.currentStatus().editOption == "child"){
	    itemParent = itemTo;
	} else {
	    itemParent = itemTo.itemParent();
	}

	const itemBlank = itemParent.bloxChildBlankNew("Item");
	// Set itemsCenter because itemBlank is not from child() of class Item
	// itemsBlank.itemsCenter(), eg. itemsBlank.itemsCenterV
	// does not have a value yet.
	itemBlank.itemsCenter(this.item().itemsCenter());

	if(this.currentStatus().editOption == "before"){
	    itemBlank.itemNext(itemTo);
	}

	// itemNext
	if(this.currentStatus().editOption == "after"){
	    const itemNext = itemTo.itemNext();
	    if(itemNext){ itemBlank.itemNext(itemNext); }
	}
	
	if(this.currentStatus().editOption == "child"){
	    const ele = itemTo.index().ele();
	    // index draw, target to draw editor
	    if(!ele){
		itemTo.index().eleDraw({"drawWithoutChild": true});
	    }
	}
	
	const indexItemBlank = itemBlank.indexItem();
	indexItemBlank.currentStatus().editOption =
	    this.currentStatus().editOption;

	// close current editor to let open editor of indexItemBlank
	this.editorClose();
	
	indexItemBlank.eleDraw();
	
	this.menu().editorOpen(indexItemBlank);
	
    } // end of class IndexItemEditor editorInsert 
    
    editorEnterNew() {
	// this.log2("editorEnterNew()","");

	const itemBlank = this.item();

	if(this.result().err.length == 0){
	    const itemParent = itemBlank.itemParent();
	    let itemNew;
	    const itemNext = itemBlank.itemNext();
	    if(itemNext){
		const option = {
		    "target" : itemNext,
		    "before" : true,
		};
		itemNew = itemParent.childNew(option);
	    } else {
		itemNew = itemParent.childNew();
	    }

	    itemNew.data().title = itemBlank.data().title;
	    itemNew.data().href = itemBlank.data().href;

	    itemBlank.childrenEleClear();

	    itemNew.eleDraw();

	    super.editorEnter();
	    
	} else {
	    // some err
	    
	    super.editorEnter();

	    // this call make this.result().err clear
	    // so do not call this before super.editorEnter()
	    // otherwise editor will be closed with no err
 	    this.item().eleDraw();
	}	

    } // end of class IndexItemEditor editorEnterNew

    editorNewPage() {
	// this.log("editorNewPage()");

	// this.result() will be cleared after this.editorEnter();
	const err = this.result("err");
	
	this.editorEnter(...arguments);

	this.log("editorNewPage() err.length:" + err.length);
	if(0 < err.length){ return; }	

	const titleNew = this.item().data().title;
	if(titleNew.length == 0){ return; }
	
	const hrefNew = this.item().data().href;
	if(hrefNew.length == 0){ return; }
	if(hrefNew.match(/^#/)){ return; }

	const data = {};
	data["title"] = titleNew;
	data["href"] = hrefNew;

	const res = postData("page_new", data);
	res.then(data => {
	    console.log("wc.jp newPage res:" + data.res);
	});
	
	
    } // end of class IndexItemEditor editorNewPage 

} // end of class IndexItemEditor end 

// class Contents
// class ContentsEditor
// class Content : a part of class Contents
//
// class Contents represents contents of class Item.
// Contents have some class Content. 
class Contents extends Blox {

    item() {
	const parentBx = this.parentBx();
	if(parentBx.constructor.name == "Item"){ return parentBx; }
	
	if(parentBx.item){ return parentBx.item(); }
	
	if(0 < arguments.length){ this.itemV = arguments[0];}
	return this.itemV;
    } // end of class Contents item 

    eleDrawInst() {

	if(! this.item().hrefStartsSharp()){ return; }

	const ele = document.createElement("div");

	// navi
	ele.appendChild(this.eleNavi());
	
	// id
	const id = this.item().data().href.replace(/^#/,  "");
	ele.setAttribute("id", id);

	// title
	const eleTitle = document.createElement("div");
	eleTitle.setAttribute('class', "subsectionTitle");
	eleTitle.appendChild(document.createTextNode(this.item().data().title));

 	ele.appendChild(eleTitle);

	// editor target
	const eleEditor = document.createElement("div");
	const keyEditor = this.editor().eleTargetName();
	eleEditor.classList.add(keyEditor);
	ele.appendChild(eleEditor);

	// contents target
	const eleContents = document.createElement("div");
	eleContents.setAttribute('class', this.bloxPrefix("contentsTarget"));
 	ele.appendChild(eleContents);
	
	this.ele(ele);

	this.editor().bloxDrawn({"drawn": true});

	this.contentsDraw();
	
    } // end of class Contents eleDrawInst

    eleNavi() {

	// navi
	const eleNavi = document.createElement("div");
	// navi back
	let backA = document.createElement('a');
	backA.setAttribute('href', "javascript:history.back()");
	backA.appendChild(document.createTextNode("back"));
	eleNavi.appendChild(backA);

	eleNavi.appendChild(document.createTextNode(" "));

	// navi top
	let topA = document.createElement('a');
	topA.setAttribute('href', "#");
	topA.appendChild(document.createTextNode("Top"));
	eleNavi.appendChild(topA);
	
	return eleNavi;	

    } // end of class Contents eleNavi 

    contentsDraw() {
	// this.log2("contentsDraw()");
	
	const dataContents = this.item().data().content;

	for(let i=0; i<dataContents.length; i++){
	    const content = this.content(i);
	    content.eleDraw();
	}

	// draw content blank to let open editor of the first content
	if(dataContents.length == 0){
	    this.contentBlankDraw();
	}

    } // end of class Contents contentsDraw

    contentBlankDraw() {

	// class ContentEditor editorInsert requires a Content as a base.
	// so editorInsert can not be used.
	const contentBlank = this.contentBlank();
	contentBlank.data().type = "text";
	contentBlank.data().value = "new content";
	contentBlank.eleDraw();
	
    } // end of class Contents contentBlankDraw 

    targetNext() {

	// first child
	const children = this.item().children()
	if(0 < children.length){
	    const item = children[0];
	    const contents = item.bloxChild("Contents", "0");

	    if(contents){
		const ele = contents.ele();
		if(ele){ return ele; }
		return contents.targetNext();
	    }
	}

	// sibling
	let itemNext = this.item().itemNext();
	// let itemNext = this.parentBx().itemNext();
	while(itemNext){
	    const contents = itemNext.bloxChild("Contents", "0");
	    if(contents){
		const ele = contents.ele();
		if(ele){ return ele; }
		return contents.targetNext();
	    }
	    itemNext = itemNext.itemNext();
	}
	
	let parent = this.item().parentBx();
	while(parent){
	    // start at itemNext	    
	    // start at parent makes endless loop 
	    let itemNext = parent.itemNext();
	    while(itemNext){
		const contents = itemNext.bloxChild("Contents", "0");
		if(contents){
		    const ele = contents.ele();
		    if(ele){ return ele; }
		    
		    const ele2 = contents.targetNext();
		    if(ele2){ return ele2; }
		}
		
		itemNext = itemNext.itemNext();
	    }
	    parent = parent.parentBx();
	    if(parent.constructor.name != "Item"){ break; }
	}

    } // end of class Contents targetNext 
    
    eleTargetChild() {
	const child = arguments[0];
	if(child == undefined){ return; }

	if(child.constructor.name == "Content"){
	    return this.querySelectorBx(this.ele(), "contentsTarget");
	}

    } // end of class Contents eleTargetChild 

    content() {
	const index = arguments[0];
	if(index == undefined){ return; }

	const dataContents = this.item().data().content;
	if(dataContents.length <= index){ return; }
	const content = this.bloxChild("Content", index, {"create" : true});
	content.contents(this);
	return content;
	
    } // end of class Contents content

    contentBlank() {
	const content = this.bloxChild("Content",
				       this.idBlank, {"create" : true});
	content.contents(this);
	return content;
    } // end of class Contents contentBlank 

    // clear all Content in bloxChild
    contentsRemove() {
	// eleChildDelete
	const bloxChild = this.bloxChild();
	for(const className in bloxChild){
	    // only instance of class Content
	    if(className != "Content"){ continue; }
	    for(const key in bloxChild[className]){
		this.bloxChild(className, key, {"remove": true});
	    }
	}
    } // end of class Contents contentsRemove

    // this.contentMove(contentThis, contentTo, option);
    // option: {"before" : true} / {"after" : true}
    contentMove() {

	const contentThis = arguments[0];
	const contentTo = arguments[1];
	const option = arguments[2];

	const thisIndex = contentThis.key();
	let targetIndex = contentTo.key();

	// remove
	const thisElement = this.item().data().content.splice(thisIndex, 1)[0];

	// thisIndex was removed, compentate the change on targetIndex
	if(thisIndex < targetIndex){ targetIndex--; }
	
	if(option.after){ targetIndex++; }
	
	// put
	this.item().data().content.splice(targetIndex, 0, thisElement);

    } // end of class Contents contentMove

    contentInsert() {
	// this.log("contentInsert()");
	
	const contentThis = arguments[0];
	const contentTo = arguments[1];
	const option = arguments[2];

	let targetTo;
	if(contentTo){
	     targetTo = contentTo.key();
	} else {
	    targetTo = 0;
	}

	if(option.after){ targetTo++; }

	// put
	this.item().data().content.splice(targetTo, 0, contentThis.data());
	
    } // end of class Contents contentInsert 

} // end of class Contents end 
    
class ContentsEditor extends Editor {
    
    item() {
	const parentBx = this.parentBx();
	if(parentBx.constructor.name == "Item"){ return parentBx; }
	
	if(parentBx.item){ return parentBx.item(); }
	
	if(0 < arguments.length){ this.itemV = arguments[0];}
	return this.itemV;
    } // end of class ContentsEditor item 

    eleDrawInst() {
	// this.log("eleDrawInst()");

	let html = this.htmlEditorBox;
	html = this.htmlPhReplace(html, this.htmlEditorTextarea);
	html = this.htmlPhReplace(html, this.htmlEditorEnter);

	let ele = this.eleFromHtml(html);
	this.eleVisibleSet(ele, {"editorNewPage" : 0});
	this.dataSet(ele);
	this.ele(ele);
	
    } // end of class ContentsEditor eleDrawInst 
    
    dataSet(ele) {

	const eleTgt = this.querySelectorBx(ele, "editorContent");
	eleTgt.textContent = JSON.stringify(this.item().data()["content"]);
	
    } // end of class ContentsEditor dataSet 

    editorEnter() {
	// this.log("editorEnter()");

	const contentsEle = this.querySelectorBx(this.ele(), "editorContent");
	// this.data()["content"] = contentsEle.value;
	const f0 = new Function('return '+ contentsEle.value +';');
	this.item().data()["content"] = f0();

	this.result("changed", true);

	super.editorEnter();

	// needs to clear eles,
	// otherwise targetNext returns old element
	// and draw new ele on it that can not be seen
	this.item().childrenEleClear();
	
	this.item().contents().eleDraw();
	
    } // end of class ContentsEditor editorEnter 
    
} // end of class ContentsEditor end 

// class Content
// class ContentEditor
// 
// class Content is an element of a class Contents for a class Item
// class Content extends PageBlox {
class Content extends Blox {

    item() {
	const parentBx = this.parentBx();
	if(parentBx.constructor.name == "Item"){ return parentBx; }
	
	if(parentBx.item){ return parentBx.item(); }
	
	if(0 < arguments.length){ this.itemV = arguments[0];}
	return this.itemV;
    } // end of class Content item 

    contents() {
	if(0 < arguments.length){ this.contentsV = arguments[0]; }
	return this.contentsV;
    } // end of class Content contents 
    
    data() {
	// this.log2("data()");
	if(this.isBlank()){ return this.dataBlank(); }
	return this.item().data().content[this.key()];
    } // end of class Content data

    dataBlank() {
	if(this.dataBlankV == undefined){ this.dataBlankV = {}; }
	return this.dataBlankV;
    } // end of class Content dataBlank 

    eleDrawInst() {
	// this.log2("eleDrawInst()");

	const type = this.data().type;

	let ele;
	if(type == "html"){ ele = this.eleHtml(); }
	if(type == "script"){ ele = this.eleScript(); }
	if(type == "text"){ ele = this.eleText(); }
	
	// editor target
	const eleEditor = document.createElement("div");
	const keyEditor = this.editor().eleTargetName();
	eleEditor.classList.add(keyEditor);
	ele.appendChild(eleEditor);

	this.ele(ele);
	
    } // end of class Content eleDrawInst 

    eleHtml() {

	let ele = document.createElement('div');
	ele.setAttribute('class', "html subsectionContent");
	ele.innerHTML = this.data().value;

	return ele;
	
    } // end of class Content eleHtml 

    eleScript() {

	let ele = document.createElement('div');
	ele.setAttribute('class', "html subsectionContent");

	let eleScript = document.createElement('div');
	eleScript.setAttribute('class', "script");
	eleScript.innerHTML = this.dataToText(this.data().value);

	ele.appendChild(eleScript);

	return ele;
	
    } // end of class Content eleScript 

    eleText() {
	// this.log2("eleText()","");

	let ele = document.createElement('div');
	ele.setAttribute('class', "html subsectionContent");

	ele.innerHTML = this.dataToText(this.data().value);

	// this.log2("eleText()","ele.innerHTML:" + ele.innerHTML);
	
	return ele;
	
    } // end of class Content eleText 

    dataToText(data) {

	// let data = this.data()["value"];

	// Convert \< or \> to &lt;, &gt;
	let str = textAngleToEntity(data);

	// <> are handled as html element.
	// But any space is handled as text,
	// espacially \n will be converted to <br>
	// Considering html, \n between element eg; <>\n<>
	// should not be handle as <br> and be ignored.
	// Convert >\n< to >< removing \n, spaces around \n as well .
	// But >\n\n< will not be ignored .
	// It is considered as intended to put returns between the elements .
	// eg: <hr>\n\n<hr>
	// str = str.replace(/>\s+</, "><");
	// "> \n <" to "><"
	str = str.replaceAll(/>[ ]*\n[ ]*</g, "><");
	// Since two \n required to set <br>,
	// remove one \n so that one return can be set.
	// otherwise two \n is minimum returns .
	// ">\n" to ">"
	// ">\n\n" to ">\n"
	str = str.replaceAll(/>\n/g, ">");

	// after here ">\n" was ">\n\n" originaly

	// // Convert ##..$ to xxx, in <... href="##..$">
	// str = this.page.href_reference().href_set(str);

	// Convert \n \n\n to <br> <p></p>
	str = text_to_html2(str);

	return str;

    } // end of class Content dataToText 

    contentNext() {

	if(this.isBlank()){ return this.contentNextBlank(...arguments); }

	const dataContents = this.item().data().content;
	let thisFound;
	for(let i=0; i<dataContents.length; i++){
	    if(i == this.key()){
		thisFound = true;
		continue;
	    }
	    // next of this
	    if(thisFound){
		return this.contents().content(i);
	    }
	}
	
    } // end of class Content contentNext 

    contentNextBlank() {
	if(0 < arguments.length){ this.contentNextBlankV = arguments[0]; }
	return this.contentNextBlankV;
    } // end of class Content contentNextBlank 
    
    targetNext() {
	const contentNext = this.contentNext();
	if(contentNext == undefined){ return; }

	return contentNext.ele();
	
    } // end of class Content targetNext 

    // this.move(contentTo, moveOption);
    move() {
	// this.log("move()");

	this.contents().contentMove(this, ...arguments);
	
    } // end of class Content move

    insert() {
	this.contents().contentInsert(this, ...arguments);
    } // end of class Content insert 
    
} // end of class Content end 

class ContentEditor extends Editor {
    
    item() {
	const parentBx = this.parentBx();
	if(parentBx.constructor.name == "Item"){ return parentBx; }
	
	if(parentBx.item){ return parentBx.item(); }
	
	if(0 < arguments.length){ this.itemV = arguments[0];}
	return this.itemV;
    } // end of class ContentEditor item 

    content() {
	return this.parentBx();
    } // end of class ContentEditor content 

    data() {
	const content = this.parentBx();
	return content.data();
    } // end of class ContentEditor data 

    eleDrawInst() {
	// this.log("eleDrawInst()");

	if(this.currentStatus().editType){
	    return this.eleDrawInter();
	}
	
	let html = this.htmlEditorBox;
	html = this.htmlPhReplace(html, this.htmlEditorContentType);
	html = this.htmlPhReplace(html, this.htmlEditorTextarea);
	html = this.htmlPhReplace(html, this.htmlEditorEnter);
	html = this.htmlPhReplace(html, this.htmlEditorInter);

	let ele = this.eleFromHtml(html);
	
	this.dataSet(ele);
	
	this.eleVisibleSet(ele, {"editorNewPage" : 0});
	
	if(this.isBlank()){
	    this.eleVisibleSet(ele, {"editorMoveMenu" : 0});
	}

	this.ele(ele);
	
    } // end of class ContentEditor eleDrawInst 

    dataSet(ele) {
	// this.log("dataSet()");

	// type
	for(let name of ["html", "text", "script"]){
	    if(name == this.content().data().type){
		const opName = "contentType" + firstUpper(name);
		const contentEle = this.querySelectorBx(ele, opName);
		contentEle.checked = true;
		
		break;
	    }
	}

	// value
	const eleTgt = this.querySelectorBx(ele, "editorContent");
	eleTgt.textContent = this.content().data().value;

    } // end of class ContentEditor dataSet 

    eleDrawInter() {
	// this.log("eleDrawInter()");

	super.eleDrawInter();

	// no child
	// before super.eleDrawInter(), this.ele() does not exist
	this.eleVisibleSet(this.ele(), {'editorOptionSetChild' : 0});
	
    } // end of class ContentEditor eleDrawInter

    editorClose() {
	super.editorClose(...arguments);

	// draw conten blank to let open editor of the first content
	const contents = this.content().contents();
	const dataContents = contents.item().data().content;
	if(dataContents.length == 0){
	    contents.contentBlankDraw();
	}

    } // end of class ContentEditor editorClose 
    
    editorEnter(blox) {
	// this.log("editorEnter()");

	// const typeEnter = this.querySelectorBx(this.ele(), "contentType").value;

	let contentType2Escaped = this.bloxPrefixEscaped("contentType2");
	const typeEnter = this.ele().querySelector('input[name='+contentType2Escaped+']:checked').value;
	
	// this.data() to this.content().data();
	const data = this.content().data();

	if(data.type != typeEnter){
	    data.type = typeEnter;
	    this.result("changed", true);
	}

	const eleValue = this.querySelectorBx(this.ele(), "editorContent");
	const valueEnter = eleValue.value;

	if(data.value != valueEnter){
	    data.value = valueEnter;
	    this.result("changed", true);
	}

	if(data.value.length == 0){
	    this.result("err","No content!");
	}

	if(this.isBlank()){ return this.editorEnterNew(); }
	
	super.editorEnter();

	const content = this.parentBx();
	content.eleDraw();

    } // end of class ContentEditor editorEnter

    onInterPrecheck(event, bloxFm, bloxTo) {
	// this.log2("onInterPrecheck");

	let result = super.onInterPrecheck(...arguments);
	if(! result){ return; }

	// confirm same Contents
	const contentsFm = this.content().contents();
	const contentsTo = bloxTo.contents();
	if(contentsFm != contentsTo){ return; }

	return true;

    } // end of class ContentEditor onInterPrecheck 
    
    editorMove(bloxTo) {
	// this.log("editorMove()");

	const contentTo = bloxTo;
	
	// moveOption
	const moveOption = {};
	// "after" / "before" / ("child")
	moveOption[this.currentStatus().editOption] = true;

	this.content().move(contentTo, moveOption);
	this.result("changed", true);

	// content does not have order data
	// once change order of contents
	// need to reflesh all contents and draw those
	const contents = this.content().contents();
	contents.contentsRemove();
	contents.contentsDraw();

    } // end of class ContentEditor editorMove 

    // this.editorInsert(bloxTo);
    editorInsert() {
	// this.log2("editorInsert()", "");

	const bloxTo = arguments[0];
	
	const contentBlank = this.content().contents().contentBlank();
	contentBlank.data().type = "text";
	contentBlank.data().value = "new content";

	if(this.currentStatus().editOption == "before"){
	    contentBlank.contentNext(bloxTo);
	} else {
	    contentBlank.contentNext(bloxTo.contentNext());
	}

	contentBlank.currentStatus().editOption =
	    this.currentStatus().editOption;
	contentBlank.currentStatus().bloxTo = arguments[0];
	
	// close current editor to let open editor of contentBlank
	this.editorClose();
	
	contentBlank.eleDraw();

	this.menu().editorOpen(contentBlank);
	
    } // end of class ContentEditor editorInsert 
    
    editorEnterNew() {
	// this.log("editorEnterNew()");

	if(0 < this.result().err.length){
	    super.editorEnter();
	    return;
	}
	
	const contentBlank = this.content();

	const contentTo = contentBlank.currentStatus().bloxTo;
	
	const option = {};
	option[contentBlank.currentStatus().editOption] = true;

	this.content().insert(contentTo, option);
	
	this.result("changed", true);
	
	this.editorClose();
	
	// content does not have order data
	// once change order of contents
	// need to reflesh all contents and draw those
	const contents = this.content().contents();
	contents.contentsRemove();
	contents.contentsDraw();


    } // end of class ContentEditor editorEnterNew

    editorDeleteExecute() {
	// this.log("editorDeleteExecute()");

	// remove
	const dataContents = this.item().data().content;
	dataContents.splice(this.content().key(), 1);		
	this.result("changed", true);
	this.editorClose();

	// content does not have order data
	// once change order of contents
	// need to reflesh all contents and draw those
	const contents = this.content().contents();
	contents.contentsRemove();
	contents.contentsDraw();
	
    } // end of class ContentEditor editorDeleteExecute 
    
} // end of class ContentEditor end 

// Convert escaped < ("\<") and > ("\>") to entiry references
// \<: &lt;,
// \>: &gt;,
// But considering \\, eg \\< is not converted .
function textAngleToEntity(str) {
    // console.log("wc.js function textAngleToEntity");

    return str.replace(textAngleRegex, textAngleToEntityReplacer);

} // end of function textAngleToEntity

// const textAngleRegex = /(\\*)([<|>])/;
const textAngleRegex = /(\\*)((?:\\<)|(?:\\>))/g;
function textAngleToEntityReplacer() {

    // arguments[0]: hole of the match
    // arguments[1]: \*
    // arguments[2]: \<|\>
    // arguments[]: 
    // arguments[]:

    // arguments[2] is with \ (\< or \>) ,
    // so if arguments[1].length is odd,
    // that means numbers of \ in arguments[1] + arguments[2] is even .
    // even: 1 means \\< , \\ (escaped \) and <
    // even: 0 means \<  , < is escaped by \ (\<)
    let even = arguments[1].length % 2;

    if(even){ return arguments[0]; }

    if(arguments[2] == "\\<"){ return arguments[1] + "&lt;"; }
    
    if(arguments[2] == "\\>"){return arguments[1] + "&gt;";}	
    
    return arguments[0];

} // end of function textAngleToEntityReplacer

// convert text data to HTML.
// space and tab to <pre class="inline0">space and tab</pre>
// \n to <br>
// // \n\n to <p>a</p><p>b</p>
// content1\n\ncontent2 to  <p>contetnt1</p> <p>contetnt2</p>
const p_start = "<p>";
const p_end = "</p>";
function text_to_html2(text) {
    // console.log("wc.js function text_to_html2 text: " + text);

    // <>\n<> shild be handled as <><>
    // any space between > : end of tag and < : start of tag should be ignore

    // text = text.replace(/[ \t]{2,}|\t+/g, '<pre class="inline0">$&</pre>');

    // use span to handle spaces to be drawn
    // since pre is blox element, it makes returns before and after pre element,
    // so not use pre but span
    // "white-space: pre" is not pre element, but a style
    text = text.replace(/[ \t]{2,}|\t+/g, '<span style="white-space: pre">$&</span>');

    let html = p_start;
    let list = text.split("\n");
    while (0 < list.length) {
	let line = list.shift();
	html += line;
	// Next line exists means \n exists.
	if (0 < list.length) {
	    // \n   : <br>
	    // \n\n : <p></p>
	    //
	    // Next line is emply.
	    // Means \n\n
	    if (list[0].length == 0) {
		// Remove next one since empty.
		list.shift();
		html += (p_end + p_start);
	    } else {
		html += "<br>"
	    }
	}
    }

    html += p_end;
    
    return html;
    
} // end of function text_to_html2

// abcDef to AbcDef
function firstUpper() {
    let name = arguments[0];
    if(name == undefined){ return name; }
    if(name.length == 0){ return name; }
    if(name.length == 1){ return name.toUpperCase(); }
    return name[0].toUpperCase() + name.slice(1);
} // end of function firstUpper

// window.scrollTo does not save the crurrent page
// and it can not be back by javascript:history.back()
function scrollHash(id) {
    // console.log("wc.js function scrollHash");

    let eleTarget = id ? document.getElementById(id) : undefined;

    eleTarget.scrollIntoView({behaivor: 'smooth'});
    // console.log("wc.js function scrollHash to:" + id);
    return;
    
    let targetRect = eleTarget ? eleTarget.getBoundingClientRect() : undefined;
    if(targetRect){
	window.scrollTo({
	    left: targetRect.left,
	    top: targetRect.top,
	    behavior: 'smooth'
	    // auto does not work
	    // behavior: 'auto'
	});

	// These do not work
	// window.scrollTo(targetRect.left, targetRect.top);
	// window.scrollTo(10, 10);
	
    }
    
} // end of function scrollHash

// Consider to send rev no .
// If rev no sent by posData and the rev no of file are not same,
// data conflict might happen .
//
async function postData(req, data) {
    const response = await fetch(
	document.URL,
	{
	    method: 'POST',
	    headers: {
		'Content-Type': 'application/json',
		'wc-request' : req,
	    },
	    body: JSON.stringify(data),
	},
    )

    return response.json();
} // end of function postData"####
}

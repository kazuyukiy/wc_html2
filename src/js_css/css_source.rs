pub fn contents() -> &'static str {
    r####"

.invisible { display: none; }

.subsection {
    margin-bottom: 20px;
}
.subsectionTitle {
    font-weight: bold;
    font-size: 1.5em;
}
.subsectionContent {
    margin-left: 30px;
}
.subsectionSub {
    margin-left: 30px;
}
.inline0 {
    display: inline;
}
.script {
    border: solid 1px black;
    /* width: 400px; */
    background: #e6e6fa;
    padding: 10px;
    resize: both;
    overflow: hidden;
    margin-bottom: 3px;
}

.editSubsectionContent {
    width: 98%;
}

.tableBorder {
    border: 1px black solid;
    border-collapse: collapse;
}
.tableBorder td {
    padding: 5px 10px 5px 10px;
    border: 1px black solid;
    vertical-align: top;
}

.tableNoBorder {
    border-collapse: collapse;
}
.tableNoBorder td{
    padding: 5px 10px 5px 10px;
    vertical-align: top;
}


.tableCollapse1 {
    border: 1px black solid;
    border-collapse: collapse;
}
.tableCollapse1 td{
    padding: 5px 10px 5px 10px;
    border: 1px black solid;
}

.editTable {
    border: solid black 1px;
}

.editTable2 {
    border: solid red 3px;
}
.editTable td {
    vertical-align: top;
}
.editDeleteConfirm {
    display: inline-block;
    border: solid black 1px;
}

.invisible {
    display: none;
}

.border2 {
    border: solid black 2px;
}
      
.textareaBig {
    width: 800px;
    height: 200px;
}
"####
}

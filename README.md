# flow_rs

Generate flowcharts from pseudocode!


## Language Grammar

The language is defined by the [parsing expression grammar](src/def.pest).


## Sample

This code
```
commute to work;
while lunchtime has not arrived {
  goof off;
}
get lunch;
eat lunch;
while the time is before 4pm {
  do work;
}
if it's friday {
  go to happy hour;
} else {
  go home;
}
```
generates this DOT
```
strict digraph {
n0 [label="commute to work", shape="rectangle", ];
n1 [shape="diamond", label="lunchtime has not arrived ", ];
n0 -> n1;
n2 [shape="rectangle", label="goof off", ];
n1 -> n2 [label="True"];
n2 -> n1;
n3 [shape="rectangle", label="get lunch", ];
n1 -> n3 [label="False"];
n4 [label="eat lunch", shape="rectangle", ];
n3 -> n4;
n5 [label="the time is before 4pm ", shape="diamond", ];
n4 -> n5;
n6 [label="do work", shape="rectangle", ];
n5 -> n6 [label="True"];
n6 -> n5;
n7 [label="it's friday ", shape="diamond", ];
n5 -> n7 [label="False"];
n8 [shape="rectangle", label="go to happy hour", ];
n7 -> n8 [label="True"];
n9 [label="go home", shape="rectangle", ];
n7 -> n9 [label="False"];
n8 [color=red, penwidth=3, shape="rectangle", label="go to happy hour", ];
n9 [label="go home", color=red, penwidth=3, shape="rectangle", ];
}
```
The resulting flowchart rendered with `dot`:
![Work flowchart](doc/work_sample.png?raw=true)

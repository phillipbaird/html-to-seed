use lazy_static::lazy_static;

use std::collections::HashMap;

pub fn seed_attr_name(name: &str) -> String {
    match ATTRIBUTE_MAP.get(name.to_lowercase().as_str()) {
        Some(mapped_name) => mapped_name.to_string(),
        None => String::from(name),
    }
}

macro_rules! map(
    { $($value:expr => $key:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

lazy_static! {
    /// A map of Html attribute strings to Seed At constructor names.
    /// This list only includes attribute names that cannot simply be mapped
    /// by title-casing the hyphenated parts of their name.
    /// e.g. We can map "accept-charset" to "AcceptCharset" with a heuristic
    /// but not "accesskey" to "AccessKey".
    static ref ATTRIBUTE_MAP: HashMap<&'static str, &'static str> = map! {
        // Html Attributes
        "Accept" => "accept", "AcceptCharset" => "accept-charset", "AccessKey" => "accesskey", "Action" => "action",
        "Alt" => "alt", "Async" => "async", "AutoComplete" => "autocomplete", "AutoFocus" => "autofocus",
        "AutoPlay" => "autoplay", "Charset" => "charset", "Checked" => "checked", "Cite" => "cite",
        "Class" => "class", "Color" => "color", "Cols" => "cols", "ColSpan" => "colspan",
        "Content" => "content", "ContentEditable" => "contenteditable", "Controls" => "controls", "Coords" => "coords",
        "Data" => "data", "DateTime" => "datetime", "Default" => "default", "Defer" => "defer",
        "Dir" => "dir", "DirName" => "dirname", "Disabled" => "disabled", "Download" => "download",
        "Draggable" => "draggable", "DropZone" => "dropzone", "EncType" => "enctype", "For" => "for",
        "Form" => "form", "FormAction" => "formaction", "Headers" => "headers", "Height" => "height",
        "Hidden" => "hidden", "High" => "high", "Href" => "href", "HrefLang" => "hreflang",
        "HttpEquiv" => "http-equiv", "Id" => "id", "IsMap" => "ismap", "Kind" => "kind",
        "Label" => "label", "Lang" => "lang", "List" => "list", "Loop" => "loop",
        "Low" => "low", "Max" => "max", "MaxLength" => "maxlength", "Media" => "media",
        "Method" => "method", "Min" => "min", "Multiple" => "multiple", "Muted" => "muted",
        "Name" => "name", "NoValidate" => "novalidate", "OnAbort" => "onabort", "OnAfterPrint" => "onafterprint",
        "OnBeforePrint" => "onbeforeprint", "OnBeforeUnload" => "onbeforeunload", "OnBlur" => "onblur", "OnCanPlay" => "oncanplay",
        "OnCanPlayThrough" => "oncanplaythrough", "OnChange" => "onchange", "OnClick" => "onclick", "OnContextMenu" => "oncontextmenu",
        "OnCopy" => "oncopy", "OnCueChange" => "oncuechange", "OnCut" => "oncut", "OnDblClick" => "ondblclick",
        "OnDrag" => "ondrag", "OnDragEnd" => "ondragend", "OnDragEnter" => "ondragenter", "OnDragLeave" => "ondragleave",
        "OnDragOver" => "ondragover", "OnDragStart" => "ondragstart", "OnDrop" => "ondrop", "OnDurationChange" => "ondurationchange",
        "OnEmptied" => "onemptied", "OnEnded" => "onended", "OnError" => "onerror", "OnFocus" => "onfocus",
        "OnHashChange" => "onhashchange", "OnInput" => "oninput", "OnInvalid" => "oninvalid", "OnKeyDown" => "onkeydown",
        "OnKeyPress" => "onkeypress", "OnKeyUp" => "onkeyup", "OnLoad" => "onload", "OnLoadedData" => "onloadeddata",
        "OnLoadedMetaData" => "onloadedmetadata", "OnLoadStart" => "onloadstart", "OnMouseDown" => "onmousedown",
        "OnMouseMove" => "onmousemove", "OnMouseOut" => "onmouseout", "OnMouseOver" => "onmouseover", "OnMouseUp" => "onmouseup",
        "OnMouseWheel" => "onmousewheel", "OnOffline" => "onoffline", "OnOnline" => "ononline", "OnPageHide" => "onpagehide",
        "OnPageShow" => "onpageshow", "OnPaste" => "onpaste", "OnPause" => "onpause", "OnPlay" => "onplay",
        "OnPlaying" => "onplaying", "OnPopState" => "onpopstate", "OnProgress" => "onprogress", "OnRateChange" => "onratechange",
        "OnRest" => "onreset", "OnResize" => "onresize", "OnScroll" => "onscroll", "OnSearch" => "onsearch",
        "OnSeeked" => "onseeked", "OnSeeking" => "onseeking", "OnSelect" => "onselect", "OnStalled" => "onstalled",
        "OnStorage" => "onstorage", "OnSubmit" => "onsubmit", "OnSuspend" => "onsuspend", "OnTimeUpdate" => "ontimeupdate",
        "OnToggle" => "ontoggle", "OnUnload" => "onunload", "OnVolumeChange" => "onvolumechange", "OnWaiting" => "onwaiting",
        "OnWheel" => "onwheel", "Open" => "open", "Optimum" => "optimum", "Pattern" => "pattern",
        "Placeholder" => "placeholder", "Poster" => "poster", "Preload" => "preload", "ReadOnly" => "readonly",
        "Rel" => "rel", "Required" => "required", "Reversed" => "reversed", "Rows" => "rows",
        "RowSpan" => "rowspan", "Sandbox" => "sandbox", "Scope" => "scope", "Selected" => "selected",
        "Shape" => "shape", "Size" => "size", "Span" => "span", "SpellCheck" => "spellcheck",
        "Src" => "src", "SrcDoc" => "srcdoc", "SrcLang" => "srclang", "SrcSet" => "srcset",
        "Start" => "start", "Step" => "step", "Style" => "style", "TabIndex" => "tabindex",
        "Target" => "target", "Title" => "title", "Translate" => "translate", "Type" => "type",
        "UseMap" => "usemap", "Value" => "value", "Width" => "width", "Wrap" => "wrap",


        // SVG
        "AccentHeight" => "accent-height", "Accumulate" => "accumulate", "Additive" => "additive", "AlignmentBaseline" => "alignment-baseline",
        "AllowReorder" => "allowReorder", "Amplitude" => "amplitude", "ArabicForm" => "arabic-form", "Ascent" => "ascent",
        "AttributeName" => "attributename", "AttributeType" => "attributetype", "AutoReverse" => "autoreverse", "Azimuth" => "azimumth",
        "BaseFrequency" => "basefrequency", "BaselineShift" => "baseline-shift", "BaseProfile" => "baseprofile", "Bbox" => "bbox",
        "Begin" => "begin", "Bias" => "bias", "By" => "by", "CalcMode" => "calcmode", "CapHeight" => "cap-height",
        "Clip" => "clip", "ClipPathUnits" => "clippathunits", "ClipPath" => "clip-path", "ClipRule" => "clip-rule",
        "ColorInterpolation" => "color-interpolation", "ColorInterpolationFilters" => "color-interpolation-filters",
        "ColorProfile" => "color-profile", "ColorRendering" => "color-rendering", "ContentScriptType" => "contentscripttype",
        "ContentStyleType" => "contentstyletype", "Cursor" => "cursor", "Cx" => "cx", "Cy" => "cy", "Decelerate" => "decelerate",
        "Descent" => "descent", "DiffuseConstant" => "diffuseconstant", "Direction" => "direction", "Display" => "display",
        "Divisor" => "divisor", "DominantBaseline" => "dominant-baseline", "Dur" => "dur", "Dx" => "dx", "Dy" => "dy", "EdgeMode" => "edgemode",
        "Elevation" => "elevation", "EnableBackground" => "enable-background", "End" => "end", "Exponent" => "exponent",
        "ExternalResourcesRequired" => "externalResourcesRequired", "FillOpacity" => "fill-opacity", "FillRule" => "fill-rule",
        "Filter" => "filter", "FilterRes" => "filterres", "FilterUnits" => "filterunits", "FloodColor" => "flood-color",
        "FloodOpacity" => "flood-opacity", "FontFamily" => "font-family", "FontSize" => "font-size", "FontSizeAdjust" => "font-size-adjust",
        "FontStretch" => "font-stretch", "FontStyle" => "font-style", "FontVariant" => "font-variant", "FontWeight" => "font-weight",
        "Format" => "format", "From" => "from", "Fr" => "fr", "Fx" => "fx", "Fy" => "fy", "G1" => "g1", "G2" => "g2",
        "GlyphName" => "glyph-name", "GlyphOrientationHorizontal" => "glyph-orientation-horizontal",
        "GlyphOrientationVertical" => "glyph-orientation-vertical", "GlyphRef" => "glyphRef", "GradientTransform" => "gradienttransform",
        "GradientUnits" => "gradientunits", "Hanging" => "hanging", "HorizAdvX" => "horiz-adv-x", "HorizOriginX" => "horiz-origin-x",
        "Ideographic" => "ideographic", "ImageRendering" => "image-rendering", "In" => "in", "In2" => "in2", "Intercept" => "intercept",
        "K" => "k", "K1" => "k1", "K2" => "k2", "K3" => "k3", "K4" => "k4", "KernelMatrix" => "kernelmatrix", "KernelUnitLength" => "kernelunitlength",
        "Kerning" => "kerning", "KeyPoints" => "keypoints", "KeySplines" => "keysplines", "KeyTimes" => "keytimes",
        "LengthAdjust" => "lengthadjust", "LetterSpacing" => "letter-spacing", "LightingColor" => "lighting-color",
        "LimitingConeAngle" => "limitingconeangle", "Local" => "local", "MarkerEnd" => "marker-end", "MarkerMid" => "marker-mid",
        "MarkerStart" => "marker-start", "MarkerHeight" => "markerheight", "MarkerUnits" => "markerunits",
        "MarkerWidth" => "markerwidth", "Mask" => "mask", "MaskContentUnits" => "maskcontentunits", "MaskUnits" => "maskunits",
        "Mathematical" => "mathematical", "Mode" => "mode", "NumOctaves" => "numoctaves", "Offset" => "offset",
        "Opacity" => "opacity", "Operator" => "operator", "Order" => "order", "Orient" => "orient", "Orientation" => "orientation",
        "Origin" => "origin", "Overflow" => "overflow", "OverlinePosition" => "overline-position", "OverlineThickness" => "overline-thickness",
        "Panose1" => "panose-1", "PaintOrder" => "paint-order", "PathLength" => "pathlength", "PatternContentUnits" => "patterncontentUnits",
        "PatternTransform" => "patterntransform", "PatternUnits" => "patternunits", "Ping" => "ping", "PointerEvents" => "pointer-events",
        "Points" => "points", "PointsAtX" => "pointsatx", "PointsAtY" => "pointsaty", "PointsAtZ" => "pointsatz",
        "PreserveAlpha" => "preservealpha", "PreserveAspectRatio" => "preserveaspectratio", "PrimitiveUnits" => "primitiveunits",
        "R" => "r", "Radius" => "radius", "ReferrerPolicy" => "referrerpolicy", "RefX" => "refx", "RefY" => "refy",
        "RenderingIntent" => "rendering-intent", "RepeatCount" => "repeatcount", "RepeatDur" => "repeatdur",
        "RequiredExtensions" => "requiredextensions", "RequiredFeatures" => "requiredfeatures", "Restart" => "restart",
        "Result" => "result", "Rotate" => "rotate", "Rx" => "rx", "Ry" => "ry", "Scale" => "scale", "Seed" => "seed",
        "ShapeRendering" => "shape-rendering", "Slope" => "slope",  "Spacing" => "spacing", "SpecularConstant" => "specularconstant",
        "SpecularExponent" => "specularexponent", "Speed" => "speed", "SpreadMethod" => "spreadmethod", "StartOffset" => "startoffset",
        "StdDeviation" => "stddeviation", "Stemh" => "stemh", "Stemv" => "stemv", "StitchTiles" => "stitchtiles", "StopColor" => "stop-color",
        "StopOpacity" => "stop-opacity", "StrikethroughPosition" => "strikethrough-position", "StrikethroughThickness" => "strikethrough-thickness",
        "String" => "string", "Stroke" => "stroke", "StrokeDashArray" => "stroke-dasharray", "StrokeDashOffset" => "stroke-dashoffset",
        "StrokeLinecap" => "stroke-linecap", "StrokeLineJoin" => "stroke-linejoin", "StrokeMiterLimit" => "stroke-miterlimit",
        "StrokeOpacity" => "stroke-opacity", "StrokeWidth" => "stroke-width", "SurfaceScale" => "surfacescale", "SystemLanguage" => "systemlanguage",
        "TableValues" => "tablevalues", "TargetX" => "targetx", "TargetY" => "targety", "TextAnchor" => "text-anchor",
        "TextDecoration" => "text-decoration", "TextRendering" => "text-rendering", "TextLength" => "textlength",
        "To" => "to", "Transform" => "transform", "U1" => "u1", "U2" => "u2", "UnderlinePosition" => "underline-position",
        "UnderlineThickness" => "underline-thickness", "Unicode" => "unicode", "UnicodeBidi" => "unicode-bidi",
        "UnicodeRange" => "unicode-range", "UnitsPerEm" => "units-per-em", "VAlphabetic" => "v-alphabetic", "VHanging" => "v-hanging",
        "VIdeographic" => "v-ideographic", "VMathematical" => "v-mathematical", "Values" => "values", "VectorEffect" => "vector-effect",
        "Version" => "version", "VertAdvY" => "vert-adv-y", "VertOriginX" => "vert-origin-x", "VertOriginY" => "vert-origin-y",
        "ViewTarget" => "viewtarget", "Visibility" => "visibility", "Widths" => "widths", "WordSpacing" => "word-spacing",
        "WritingMode" => "writing-mode", "X" => "x", "XHeight" => "x-height", "X1" => "x1", "X2" => "x2",
        "XchannelSelector" => "xchannelselector", "XlinkActuate" => "xlink:actuate", "XlinkArcrole" => "xlink:arcrole",
        "XlinkHref" => "xlink:href", "XlinkRole" => "xlink:role", "XlinkShow" => "xlink:show",
        "XlinkTitle" => "xlink:title", "XlinkType" => "xlink:type", "XmlBase" => "xml:base", "XmlLang" => "xml:lang",
        "XmlSpace" => "xml:space", "Y" => "y", "Y1" => "y1", "Y2" => "y2", "YchannelSelector" => "ychannelselector",
        "Z" => "z", "ZoomAndPan" => "zoomAndPan", "Path" => "path", "D" => "d", "Xmlns" => "xmlns",
        "ViewBox" => "viewbox", "Fill" => "fill"


    };
}

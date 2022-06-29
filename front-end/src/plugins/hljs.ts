import hljs from "highlight.js/lib/core";
import javascript from "highlight.js/lib/languages/javascript";
import cpp from "highlight.js/lib/languages/cpp";
hljs.registerLanguage("javascript", javascript);
hljs.registerLanguage("cpp", cpp);

export default hljs;

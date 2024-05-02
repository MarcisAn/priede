var multilineComment = /\/\*(?:[^*/]|\*(?!\/)|\/(?!\*)|<self>)*\*\//.source;
for (var i = 0; i < 2; i++) {
  // support 4 levels of nested comments
  multilineComment = multilineComment.replace(/<self>/g, function () {
    return multilineComment;
  });
}
multilineComment = multilineComment.replace(/<self>/g, function () {
  return /[^\s\S]/.source;
});

Prism.languages.priede = {
  comment: [
    {
      pattern: RegExp(/(^|[^\\])/.source + multilineComment),
      lookbehind: true,
      greedy: true,
    },
    {
      pattern: /(^|[^\\:])\/\/.*/,
      lookbehind: true,
      greedy: true,
    },
  ],
  tag: {
    // labels
    pattern: /^([ \t]*)[^\s,`":]+(?=:[ \t]*$)/m,
    lookbehind: true,
  },
  string: /"(?:[^"\n\r]|"")*"/,
  variable: /%\w+%/,
  number: /\b0x[\dA-Fa-f]+\b|(?:\b\d+(?:\.\d*)?|\B\.\d+)(?:[Ee]-?\d+)?/,
  operator: /[-+*\/%!^]=?|=[=>]?|&[&=]?|\|[|=]?|<<?=?|>>?=?|[@?]/,
  boolean: /\b(?:false|true)\b/,

  //command: {
  //  pattern: /\b(?:AutoTrim)\b/i,
  //  alias: "selector",
  //},

  constant: /\b(?:PAT|PATIESS|NEPAT|NEPATIESS)\b/i,

  builtin: /\b(?:skaitlis|sk|būls|bl|teksts|tx)\b/i,

  //symbol: /\b(?:xbutton2)\b/i,

  directive: {
    pattern: /#[a-z]+\b/i,
    alias: "important",
  },

  keyword: /\b(?:citādi|saraksts|atgriezt|atkārtot|kamēr|funkc|ja|iekļaut)\b/i,
  function: /[^(); \t,\n+*\-=?>:\\\/<&%\[\]]+(?=\()/,
  punctuation: /[{}[\]():,]/,
};

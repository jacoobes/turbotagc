const { compile } = require('../pkg/turbotagc')

compile(String.raw`
    def test = {{ lorem \s\s ipsum }} [ a > c | d ]
`, true);


/**
 *  Expr
 *  func s(a,v,x) start
 *
 *
 *  end
 *  parenthesis -> prim
 *  prim -> prim
 *
 *
 *
 *
 *
 */
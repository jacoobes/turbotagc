const { compile } = require('../pkg/turbotagc')

compile(String.raw`
    def test = {{ lorem ipsum }} [ a > c | d ] end
    
    start 
      pooba pooba [a > v | s ] ??[ v > c | z ]
    end
`);


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
const { compile } = require('../pkg/turbotagc')

compile(String.raw`
    def test = {{ lorem \s\s ipsum }} [ a > c | d ] 
    
    start 
        a \n b \n cds df s $test
    end
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
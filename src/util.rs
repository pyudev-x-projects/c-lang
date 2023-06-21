use pyu_rust_util as pyu;

pub fn date() {
    pyu::funcs::output(pyu::funcs::exec("date", "-R"));
}

pub fn clear() {
   pyu::funcs::output(pyu::funcs::exec("clear", "-x")); 
}

pub fn lorem() {
    println!("Lorem ipsum dolor sit amet, consectetur adipisicing elit. (Latin)");
    println!("It is very important for the customer to pay attention to the undergraduate process. (English - Translated)");
}

pub fn newl() {
    println!("");
}

pub fn chair() {
    println!("chair.
                                     
   _|  _|                            
 _|_|_|_|_|                          
   _|  _|                            
 _|_|_|_|_|                          
   _|  _|                            
   _|  _|                            
 _|_|_|_|_|                          
   _|  _|                            
 _|_|_|_|_|                          
   _|  _|                            
   _|  _|      _|  _|      _|  _|    
 _|_|_|_|_|  _|_|_|_|_|  _|_|_|_|_|  
   _|  _|      _|  _|      _|  _|    
 _|_|_|_|_|  _|_|_|_|_|  _|_|_|_|_|  
   _|  _|      _|  _|      _|  _|    
   _|  _|                  _|  _|    
 _|_|_|_|_|              _|_|_|_|_|  
   _|  _|                  _|  _|    
 _|_|_|_|_|              _|_|_|_|_|  
   _|  _|                  _|  _|    
   _|  _|                  _|  _|    
 _|_|_|_|_|              _|_|_|_|_|  
   _|  _|                  _|  _|    
 _|_|_|_|_|              _|_|_|_|_|  
   _|  _|                  _|  _|    
                                     
                                     
");
}

pub fn saturn() {
  println!("                     .::.
                  .:'  .:
        ,MMM8&&&.:'   .:'
       MMMMM88&&&&  .:'
      MMMMM88&&&&&&:'
      MMMMM88&&&&&&
    .:MMMMM88&&&&&&
  .:'  MMMMM88&&&&
.:'   .:'MMM8&&&'
:'  .:'
'::' ");
}

This is what you see eventuall
```
13:41:38 ❤ cargo size --bin hello -- -A                                                                                      nix-shell 
    Finished dev [optimized + debuginfo] target(s) in 0.07s                                                                            
hello  :                                                           
section              size        addr                                                                                                  
.vector_table         256         0x0                                                                                                  
.text                9680       0x100                                                                                                  
.rodata              4556      0x26d0        
.data                   8  0x20000000                                                                                                  
.bss                16460  0x20000008                     
.uninit                 0  0x20004054                                                                                                  
.debug_str          46828         0x0               
.debug_abbrev        1709         0x0                      
.debug_info         21898         0x0                                                                                                  
.debug_aranges       1520         0x0                                                                                                  
.debug_ranges        7656         0x0
.debug_pubnames     14806         0x0                                                                                                  
.debug_pubtypes       270         0x0                                                                                                  
.ARM.attributes        48         0x0
.debug_frame         3324         0x0
.debug_line         25201         0x0
.comment              147         0x0
Total              154367
```
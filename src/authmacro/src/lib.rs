#![recursion_limit = "128"]


use proc_macro::TokenStream;
use proc_macro2::{Ident,Span};

use syn::*;
use quote::{quote};
use syn::visit_mut::{self, VisitMut};

use uuid::Uuid;

fn to_pascal_case(non_pascalized_string: String) -> String {
    let mut new_word: bool = true;
    let mut last_char: char = ' ';
    non_pascalized_string
        .chars()
        .fold("".to_string(), |mut result, character|
            if character == '-' || character == '_' || character == ' ' {
                new_word = true;
                result
            } else if character.is_numeric() {
                new_word = true;
                result.push(character);
                result
            } else if new_word || (
                (last_char.is_lowercase() && character.is_uppercase()) &&
                (last_char != ' ')
                ){
                new_word = false;
                result.push(character.to_ascii_uppercase());
                result
            } else {
                last_char = character;
                result.push(character.to_ascii_lowercase());
                result
            }
        )
}

fn generate_unique_ident(prefix: &str,initial_ident:&Ident) -> Ident {
    let uuid = Uuid::new_v4();
    let ident = format!("{}_{}_{}", prefix,initial_ident, uuid).replace('-', "_");

    Ident::new(&ident, Span::call_site())
}

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

fn add_generic_computation(mut generics: Generics, return_type: &Type,
                            authtypes: &Vec<Type>, computation_ident: &Ident) -> Generics{    

    let where_tt: WherePredicate = parse_quote! {
        #computation_ident:Computation<T = #return_type>
    };

    /*
    let where_tt_auth: WherePredicate = parse_quote! {
        C:Auth
    };

    let where_tt_authtype: WherePredicate = parse_quote! {
        C:AuthType<Sized>
    };
    */

    //let computation_ident = Ident::new("C", Span::call_site());
    generics.params.push( GenericParam::from( TypeParam::from(computation_ident.clone()) ));

    let where_clause = generics.make_where_clause();
    where_clause.predicates.push(where_tt);

    for authtype in authtypes {
        //let at = &mut authtype.clone();
        //fix_autht_in_authtype(at, computation_ident);

        let where_tt_authtype: WherePredicate = parse_quote! {
            #computation_ident:AuthType<#authtype>
        };
        
        where_clause.predicates.push(where_tt_authtype);
    }
    //where_clause.predicates.push(where_tt_auth);
    //where_clause.predicates.push(where_tt_authtype);

    generics
}


use syn::punctuated::Punctuated;
use syn::{FnArg, Pat, ItemFn, Block};

fn extract_arg_idents(fn_args: Punctuated<FnArg, syn::token::Comma>) -> Vec<Box<Pat>> { 
  return fn_args.into_iter().map(extract_arg_pat).collect::<Vec<_>>();
}

fn extract_arg_pat(a: FnArg) -> Box<Pat> {
    match a {
      FnArg::Typed(p) => p.pat,
      _ => panic!("Not supported on types with `self`!"),
    }
}

struct RecursiveCallReplace<'a>{
    authtypes: Vec<Type>,
    fn_call: &'a String,
    callname:&'a Ident,
    computation_ident:&'a Ident,
    computation_var:&'a Ident
}

impl RecursiveCallReplace<'_> {

    fn change_recursive_call (&mut self,segment: &mut PathSegment){
        segment.ident = self.callname.clone();

        let c_ident = self.computation_ident.clone();

        if segment.arguments.is_empty() {
            //FIX Include function params
            let arguments_tt: AngleBracketedGenericArguments  = parse_quote! {
                ::<#c_ident>
            };
            segment.arguments = PathArguments::AngleBracketed(arguments_tt);
        }else {
                let PathSegment{ref mut arguments,..} = segment;
                if let PathArguments::AngleBracketed(ref mut args) = arguments {
                    let arguments_tt: GenericArgument  = parse_quote! {
                        #c_ident
                    };
                    args.args.push(arguments_tt);

                }
            
        }

        //Include computation 
    }

    fn change_unauth_call (&mut self,segment: &mut PathSegment){

    }

}

impl<'a> VisitMut for RecursiveCallReplace<'a> {

    fn visit_expr_return_mut(&mut self, node: &mut ExprReturn) {
        //println!("Return {:?}",node);
        visit_mut::visit_expr_return_mut(self,node);
    }

    fn visit_fn_arg_mut(&mut self, node: &mut FnArg) {
        //println!("FN Arg -> {:?}",node);
        visit_mut::visit_fn_arg_mut(self,node);
    }

    /** 
     * 
     * */ 

    fn visit_type_mut(&mut self, node: &mut Type) {
        //println!("Type -> {:?}",node);

        //We need to visit leaves first because AuthT related changes 
        //in authtypes need to be recursive and botton up
        visit_mut::visit_type_mut(self,node);

        let c_ident = self.computation_ident.clone();

        if let Type::Path(ref mut typepath) = *node {
                    let TypePath {ref mut path,..} = typepath;
                    let Path{ ref mut segments,..}= path;

                    if let Some(ref mut segment) = segments.last_mut() {
                        if segment.ident.to_string() == "AuthT" {
                            if segment.arguments.is_empty() {
                                //FIX Include function params
                                let arguments_tt: AngleBracketedGenericArguments  = parse_quote! {
                                    ::<#c_ident>
                                };
                                segment.arguments = PathArguments::AngleBracketed(arguments_tt);
                            }else {
                                    let PathSegment{ref mut arguments,..} = segment;
                                    if let PathArguments::AngleBracketed(ref mut args) = arguments {
                                        let arguments_tt: GenericArgument  = parse_quote! {
                                            #c_ident
                                        };
                                        //Add type to form the where clause with AuthType
                                        if let Some(ref generic_argument) = args.args.first() {
                                            if let GenericArgument::Type (ref ty) = generic_argument{

                                                self.authtypes.push(ty.clone());
                                            }
                                        }
                                        

                                        args.args.push(arguments_tt);

                                    }
                                
                            }
                        }

                    }
                

            
        }

        
    }

    fn visit_return_type_mut(&mut self , node: &mut ReturnType) {
        //println!("Return Type -> {:?}",node);
        visit_mut::visit_return_type_mut(self,node);
    }

    fn visit_type_param_mut(&mut self, node: &mut TypeParam) {
        //println!("Param Type -> {:?}",node);
        visit_mut::visit_type_param_mut(self,node);

    }

    /*fn visit_pat_type_mut(&mut self, node: &mut PatType) {
        println!("Type Pat -> {:?}",node);
        visit_mut::visit_pat_type_mut(self,node);
    }*/

    
    fn visit_expr_call_mut (&mut self, node: &mut ExprCall) {

        visit_mut::visit_expr_call_mut (self, node);

        let c_var = self.computation_var.clone();
        let c_ident = self.computation_ident.clone();
        //println!("{:?}",*node.func);
        if let Expr::Path(ref mut exprpath) = &mut *node.func {
                    let ExprPath {ref mut path,..} = exprpath;
                    let Path{ ref mut segments,..}= path;

                    if let Some(ref mut segment) = segments.last_mut() {
                        //println!("{:?}",segment.arguments);

                        let s_ident = segment.ident.to_string();
                        if &s_ident == self.fn_call {

                            //replace name of recursive call
                            //including computation as generic argument
                            self.change_recursive_call(segment);

                            //insert computation as first argument
                            let c_argument: Expr = parse_quote! {
                                #c_var
                            };
                            node.args.insert(0,c_argument);
                        } else if s_ident == "unauth" {
                            let c_argument: Expr = parse_quote! {
                                #c_var
                            };
                            node.args.insert(0,c_argument);

                            *node =  parse_quote! {
                                RefCell::borrow(#c_ident::#node)
                            };                        
                        } else if s_ident == "unauth_mut"{
                            let c_argument: Expr = parse_quote! {
                                #c_var
                            };
                            node.args.insert(0,c_argument);

                            *node =  parse_quote! {
                                RefCell::borrow_mut(#c_ident::#node)
                            };  
                        } else if s_ident == "auth" {
                            *node =  parse_quote! {
                                #c_ident::#node
                            };
                        }
                           
                        
                    }
                
            
        }
        
    
    }

   
}

//TO FIX: All identifiers must be uuid C,c
fn change_code (sig: &mut Signature, block: &mut Block,fn_call:&String,callname:&Ident,
                computation_ident:&Ident,computation_var:&Ident) -> Vec<Type> {

    let mut rc = RecursiveCallReplace {
        authtypes: Vec::new(),
        fn_call: fn_call,
        callname: callname,
        computation_ident: computation_ident,
        computation_var: computation_var
    };

    rc.visit_signature_mut(sig);
    rc.visit_block_mut(block);

    rc.authtypes
}

#[proc_macro_attribute]
pub fn auth(_attr: TokenStream, function: TokenStream) -> TokenStream {

    // Parse our item, expecting a function. This function may be an actual
	// top-level function or it could be a method (typically dictated by the
	// arguments). We then extract everything we'd like to use.
	let ItemFn {
		vis,
		mut block,
		mut sig,
		attrs
	} = match syn::parse(function).expect("failed to parse tokens as a function") {
		Item::Fn(item) => item,
		_ => panic!("#[auth] can only be applied to functions"),
    };
  

    let Signature {
         ident,
         ..
    } = sig.clone();

    //Generate a new ID for inner function, computation type and computation variable
    let builder_ident = Ident::new(&to_pascal_case(format!("{}Computation", ident)), Span::call_site());

    let callname = generate_unique_ident("inner",&ident);
    let computation_ident = generate_unique_ident("Auth",&Ident::new("C", Span::call_site()));
    let computation_var = generate_unique_ident("c",&Ident::new("var", Span::call_site()));
    let proofstream_arg = generate_unique_ident("proof",&Ident::new("stream", Span::call_site()));

    let authtypes = change_code(&mut sig,&mut *block,&ident.to_string(),
                                &callname,&computation_ident,&computation_var);

    let Signature {
        unsafety,
        abi,
        constness,
        fn_token,
        ident,
        generics,
        output,
        inputs,
        ..
    } = sig;
    
    

    let (return_ty, rarrow_token) = match output {
		ReturnType::Type(rarrow_token, t) => (*t, rarrow_token),
		ReturnType::Default => (
			TypeTuple {
				elems: Default::default(),
				paren_token: Default::default(),
			}.into(),
			Default::default(),
		),
    };

    

    let body = quote! {

        let val = (|| 
            #block
        )();

        let pv = #computation_var.pure(val);
        #computation_var.bind( |_p| pv );
        #computation_var.put(val);

        val
    };

    
    let g = add_generic_computation(generics,&return_ty,&authtypes,&computation_ident);
    let where_clause = &g.where_clause;

    let c_argument = quote! {
       #computation_var:&mut #computation_ident
    };


    let output_inner = quote! {
        #(#attrs)*
        #vis #unsafety #abi #constness
        #fn_token #callname #g (#c_argument, #inputs )
                #rarrow_token #return_ty 
                #where_clause {
                
        #body
        }
    };

    let arguments = extract_arg_idents (inputs.clone());

    let test_signature =  parse_str::<Signature>("fn test () -> bool").unwrap();

    let output = quote! {
        /*
        struct #builder_ident #g 
            #where_clause
        {
            #inputs
        }
        */
        #(#attrs)*
        #vis #unsafety #abi #constness
        #fn_token #ident #g (#inputs,#proofstream_arg:Option<&ProofStream>)
                #rarrow_token #computation_ident
                #where_clause {
        
                    #output_inner
                
                let mut computation = #computation_ident::new(#proofstream_arg);
                #callname::#g (&mut computation,#(#arguments),*);
                computation
                
        }
        
    };

    //println!("{}", output);
    output.into()
}



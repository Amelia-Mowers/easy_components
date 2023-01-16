#![feature(macro_metavar_expr)]

#[macro_export]
macro_rules! implement_trait_and_macro_for_component{
    (
        $component_struct_name:ident,
        $new_trait_vis:vis $new_trait_name:ident,
        // $(($($sub_type:ident, $sub_type_label:ident),* ),)?
        $macro_name:ident
        $(, $method_name:ident($self:ty$(, $par:ident: $type:ty)*))
        // $(, $method_name:ident($self:ty$(($($sub_type_label_ref:ident),*))?$(, $par:ident: $type:ty)*))
        *
    ) => {
        // $new_trait_vis trait $new_trait_name$(: $($sub_type +)*)? {
        $new_trait_vis trait $new_trait_name {
            $(
                fn $method_name(&self$(, $par: $type)*);
                // fn $method_name(&self$(, $par: $type)*);
            )*
        }

        #[macro_export]
        macro_rules! $macro_name {
            (
                $$struct:ident,
                $$component_field_name:ident
                // $(, $($$$sub_type_label:ident),*)?
            ) => {
                impl $new_trait_name for $$struct {
                    $(
                        fn $method_name(&self$(, $par: $type)*) { 
                            $component_struct_name::$method_name(&self.$$component_field_name$(, $par)*) 
                            // $component_struct_name::$method_name(&self.$$component_field_name$($(, $sub_type_label_ref:ident)*)?$(, $par)*) 
                            // $component_struct_name::$method_name(&self.$$component_field_name$(, $(&self.$$$sub_type_label),*)?$(, $par)*) 
                        }
                    )*
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct ComponentA {}
    
    impl ComponentA {
        pub fn do_thing(&self) { println!("Hello from comp A") }
        fn do_thing_other(&self, str: String) { println!("Hola from comp A") }
    }
    
    implement_trait_and_macro_for_component!(
        ComponentA, 
        pub A, 
        implement_a_based_from_component,
        do_thing(&self), 
        do_thing_other(&self, str: String)
    );

    pub struct ComponentB {}
    
    impl ComponentB {
        // pub fn do_third_thing(&self, comp_a: ComponentA) { comp_a.do_thing() }
        pub fn do_third_thing(&self) { println!("Hello from comp B") }
    }
    
    implement_trait_and_macro_for_component!(
        ComponentB, 
        pub B,
        // (A, a_comp_field), 
        implement_b_based_from_component,
        // do_third_thing(&self(a_comp_field))
        do_third_thing(&self)
    );
    
    struct ContainerA {
        component_a: ComponentA
    }
    implement_a_based_from_component!(ContainerA, component_a);
    
    struct ContainerB {
        component_a: ComponentA,
        component_b: ComponentB
    }
    implement_a_based_from_component!(ContainerB, component_a);
    implement_b_based_from_component!(ContainerB, component_b);
    // implement_b_based_from_component!(ContainerB, component_b, component_a);
    
    // struct ContainerC {
    //     component_b: ComponentB
    // }
    // implement_b_based_from_component!(ContainerC, component_b);

    #[test]
    fn it_works() {
        let cont_a = ContainerA { component_a: ComponentA {} };
        let cont_b = ContainerB { component_a: ComponentA {}, component_b: ComponentB {} };

        cont_a.do_thing();
        cont_b.do_thing();
        cont_b.do_third_thing();
    }
}

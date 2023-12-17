
#[cfg(test)]
mod tests {

    use crate::Tree;
   
    #[test]
    #[should_panic(expected = "display")]
    fn try_print_empty_() {
        let ima_stupid : Tree = Tree::new(); 
        println!("{}", ima_stupid);
    }

    #[test]
    fn try_print_tree() {
        let mut hardtry : Tree = Tree::new();
        hardtry.insert(5);
        hardtry.insert(2);
        hardtry.insert(4);
        hardtry.insert(7);
        hardtry.insert(9);
        hardtry.insert(10);
        hardtry.insert(0);
        hardtry.insert(1);
        hardtry.insert(-2);
        
        println!("{}", hardtry);
    }
   
    #[test]
    fn try_delete_el() {
        let mut hardtry : Tree = Tree::new();
        hardtry.insert(5);
        hardtry.insert(2);
        hardtry.insert(4);
        hardtry.insert(7);
        hardtry.insert(9);
        hardtry.insert(10);
        hardtry.insert(0);
        hardtry.insert(1);
        hardtry.insert(-2);
        
        hardtry.del(2);
        hardtry.del(1);
        println!("{}", hardtry);
    }
    
    #[test]
    #[should_panic(expected = "delete")]
    fn try_delete_from_emty() {
        let mut hardtry : Tree = Tree::new();
        hardtry.del(1);
    }

    #[test]
    fn try_max() {
        let mut hardtry : Tree = Tree::new();
        hardtry.insert(5);
        hardtry.insert(2);
        hardtry.insert(4);
        hardtry.insert(7);
        hardtry.insert(9);
        hardtry.insert(10);
        hardtry.insert(0);
        hardtry.insert(1);
        hardtry.insert(-2);
        
        assert_eq!(hardtry.max(),10);
    }

    #[test]
    #[should_panic(expected = "delete")]
    fn try_max_from_empty() {
        let mut hardtry : Tree = Tree::new();
        hardtry.max();
    }
}
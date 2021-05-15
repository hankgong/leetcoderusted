/**
 * @defgroup   LOOP loop
 *
 * @brief      This file implements loop.
 *
 * @author     Hankg
 * @date       2021
 */

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        //range is not inclusive, so i goes from 0 to 4.
        for i in 0..5 {
            println!("Hello {}", i);
        }
    }

    #[test]
    fn test2(){
        for i in 0..5 {
            let even_odd = if i % 2 == 0 {"even"} else {"odd"};
            println!("{} {}", even_odd, i);
        }
    }

    #[test]
    fn test3() {
        let mut sum = 0.0;
        for i in 0..5 {
            sum += i as f64;    //need to explicitly convert
        }
        println!("{:?}", sum);
    }

    fn sqr(x: f64) -> f64 {
        x*x                     //return x*x; is OK; note here no semicolon at the end
    }

    #[test]
    fn test4() {
        let res  = sqr(2.0);    //if use 2 then compile error
        println!("square is {}", res);
    }

    fn modifies(x: &mut f64) {
        *x = 1.0;
    }

    #[test]
    fn test5() {
        let mut res = 0.0;
        modifies(&mut res);
        println!("res is {}", res);
        println!("{:?}", "snd dfd");
    }

    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }

    #[test]
    fn test6(){
        let a = [1, 2, 3];
        let mut m = [1, 2, 3];

        print_type_of(&a);
        print_type_of(&mut m);
        print_type_of(&test6);

        for e in a.iter() {
            println!("{:?}", e);
        }
    }

    #[test]
    fn test7(){
        let mut v: Vec<i32> = vec![1, 2, 3, 4];

        println!("v's capacity is {}", v.capacity());
        println!("Address of v's first element: {:p}", &v[0]);

        v.push(5);
        println!("v's capacity is {}", v.capacity());
        println!("Address of v's first element: {:p}", &v[0]);
    }
}
fn main() {

    let org_arr:[i32;8] = [1,2,3,5,6,8,10,11];
    let sub_arr:[i32;3] = [2,3,5];


    let mut check:bool = false;
    
    for i in 0..( org_arr.len() - sub_arr.len() +1 ) {

        if org_arr[i] == sub_arr[0] {

            let mut mark: bool =  true;
            let mut j = i; 
            for k in 0..sub_arr.len() {
                if sub_arr[k] != org_arr[j] {

                    mark = false;
                    break;
                }
                j = j+1;
            }
            if mark == true {
                check = true;
            }
        }
    }
    if check == true {
        println!("Yes");
    }
    else {
        println!("No");
    }
}
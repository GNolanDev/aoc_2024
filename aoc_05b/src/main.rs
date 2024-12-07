use aoc_05::file_reader::read_file_to_lines_vector_on_dblnewline;

fn main() {
    // read file into a vector of 2 vectors of strings, each string a line
    let lines = read_file_to_lines_vector_on_dblnewline("input_05.txt".to_string());

    // destructure into two variables, 'rules' and 'pages', each rule has the format "i|j" where i and j are integers, each page is a string of integers separated by commas
    let (rules, pages) = (&lines[0], &lines[1]);

    // create a hashmap of the first integers from 'rules', called firstnums, with a vector of corresponding second nums as the value
    let mut firstnums = std::collections::HashMap::new();
    for rule in rules {
        let nums: Vec<i32> = rule.split("|").map(|s| s.parse().unwrap()).collect();
        firstnums.entry(nums[0]).or_insert(vec![]).push(nums[1]);
    }

    // println!("The rules are: {:?}", firstnums);

    // convert the pages vector of strings into a vector of integers by exploding on "," and parsing each element
    let pages: Vec<Vec<i32>> = pages
        .iter()
        .map(|s| s.split(",").map(|s| s.parse().unwrap()).collect())
        .collect();

    // create a function to reject any page that does not contain a number from the firstnums hashset
    fn reject_if_not_in_firstnums(
        page: Vec<i32>,
        firstnums: &std::collections::HashMap<i32, Vec<i32>>,
    ) -> bool {
        for num in page {
            if firstnums.contains_key(&num) {
                return false;
            }
        }
        true
    }

    // create a function to reject any page where a number from the value of firstnums occurs earlier than its key
    // if rejecting, also return the index of the failing number as the second element of the tuple
    fn reject_if_secondnum_before_firstnum(
        page: Vec<i32>,
        firstnums: &std::collections::HashMap<i32, Vec<i32>>,
    ) -> (bool, i32) {
        for &num in &page {
            if firstnums.contains_key(&num) {
                let secondnums = firstnums.get(&num).unwrap();
                for &secondnum in secondnums {
                    if let Some(index) = page.iter().position(|&x| x == secondnum) {
                        if index < page.iter().position(|&x| x == num).unwrap() {
                            return (true, index as i32);
                        }
                    }
                }
            }
        }
        (false, 0)
    }

    // similar to above function, but given an index, checks just that index of the page and returns (true, n) if the number at that index is a secondnum that occurs before its firstnum
    fn has_invalid_num_before_it(
        page: Vec<i32>,
        firstnums: &std::collections::HashMap<i32, Vec<i32>>,
        index: usize,
    ) -> (bool, i32) {
        let num = page[index];
        if firstnums.contains_key(&num) {
            let secondnums = firstnums.get(&num).unwrap();
            for &secondnum in secondnums {
                if let Some(second_index) = page.iter().position(|&x| x == secondnum) {
                    if second_index < index {
                        return (true, second_index as i32);
                    }
                }
            }
        }
        (false, 0)
    }

    // create a function to take the middle element of a page vector
    fn middle_element(page: Vec<i32>) -> i32 {
        let len = page.len();
        page[len / 2]
    }

    // create a function to iterate over the pages, apply the tests and add the failing pages to the failed_pages vector
    fn collect_failing_pages(
        pages: Vec<Vec<i32>>,
        firstnums: &std::collections::HashMap<i32, Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut failed_pages = vec![];
        for page in pages {
            if !reject_if_not_in_firstnums(page.clone(), firstnums)
                && reject_if_secondnum_before_firstnum(page.clone(), firstnums).0
            {
                failed_pages.push(page);
            }
        }
        failed_pages
    }

    // create a function that takes a failing page and the rules, and puts the order right
    // it does this by checking the last element of the page using the rules to see if any element before it is in its rules value,
    // it checks the elements using has_invalid_num_before_it function
    // if it is, it swaps the positions of the two elements
    // it then checks again from the same index
    // if it passes, the second to last element is checked, and so on until the first element is reached, it then returns the page
    fn put_order_right(page: Vec<i32>, firstnums: &std::collections::HashMap<i32, Vec<i32>>) -> Vec<i32> {
        let mut page = page;
        let len = page.len();
        for i in (0..len).rev() {
            let index = i;
            while index > 0 {
                let (invalid, invalid_index) = has_invalid_num_before_it(page.clone(), firstnums, index);
                if invalid {
                    page.swap(index, invalid_index as usize);
                } else {
                    break;
                }
            }
        }      
        
        page
    }

    let failed_pages = collect_failing_pages(pages, &firstnums);
    let correct_pages: Vec<Vec<i32>> = failed_pages.iter().map(|page| put_order_right(page.clone(), &firstnums)).collect();

    // run the correct pages through the test again and see if they all pass now
    let refailed_pages = collect_failing_pages(correct_pages.clone(), &firstnums);

    // if there are any refailed pages, print them
/*     if refailed_pages.len() > 0 {
        println!("The following pages are still failing the test:");
        for page in refailed_pages {
            println!("{:?}", page);
        }
    } */

    let sum_of_middle_elements: i32 = correct_pages.iter().map(|page| middle_element(page.clone())).sum();
    println!("The sum of the middle elements of the correct pages is: {}", sum_of_middle_elements);

}

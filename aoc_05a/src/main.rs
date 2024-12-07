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

    // debug
    println!("firstnums: {:?}", firstnums);

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
    fn reject_if_secondnum_before_firstnum(
        page: Vec<i32>,
        firstnums: &std::collections::HashMap<i32, Vec<i32>>,
    ) -> bool {
        for &num in &page {
            if firstnums.contains_key(&num) {
                for &secondnum in firstnums.get(&num).unwrap() {
                    // reject if secondum doesn't occur in the page
                    if !page.contains(&secondnum) {
                        continue;
                    }
                    if page.iter().position(|&x| x == secondnum).unwrap()
                        < page.iter().position(|&x| x == num).unwrap()
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    // create a function to take the middle element of a page vector
    fn middle_element(page: Vec<i32>) -> i32 {
        let len = page.len();
        page[len / 2]
    }

    // create a function to iterate over the pages, apply the tests and sum the middle elements
    fn sum_middle_elements(
        pages: Vec<Vec<i32>>,
        firstnums: &std::collections::HashMap<i32, Vec<i32>>,
    ) -> i32 {
        let mut sum = 0;
        for page in pages {
            if !reject_if_not_in_firstnums(page.clone(), firstnums)
                && !reject_if_secondnum_before_firstnum(page.clone(), firstnums)
            {
                sum += middle_element(page);
            }
        }
        sum
    }

    // print the sum of the middle elements of the pages that pass the tests
    println!("{}", sum_middle_elements(pages, &firstnums));
}

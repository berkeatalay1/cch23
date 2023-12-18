use actix_web::{post, HttpResponse, web};

use crate::models::q6::ResElfString;


#[post("/6")]
async fn elf_string_endpoint(body: web::Bytes) -> HttpResponse {
    let elf_string = std::str::from_utf8(&body).unwrap();
    let (count_elf, count_elf_on_a_shelf,count_shelf ) = count_words(elf_string);
   
    let response = ResElfString{elf:count_elf, elf_on_a_shelf: count_elf_on_a_shelf, shelf_no_elf: count_shelf };

    HttpResponse::Ok().json(response)
}

// This function is needed because String.matches() does not return overlaping matchings.
fn count_words(s:&str) -> (usize,usize,usize) {
    let v: Vec<&str> = s.split_whitespace().collect();
    let mut i = 0;
    let mut count_elf = 0;
    let mut count_elf_on_a_shelf=0;
    let mut count_shelf=0;

    while i < v.len() {

        let current = v[i];
        let previous = if i < 3 { "".to_string()} else {v[i-3..i+1].join(" ")};
        let next = if i + 4 <= v.len() { v[i..i+4].join(" ") } else { "".to_string()};


        if current.contains("elf") {
            count_elf += 1;
            if current.contains("shelf") && (!previous.contains("elf on a shelf")){
                count_shelf += 1;
            }
            if next.contains("elf on a shelf"){
                count_elf_on_a_shelf += 1;
                i += 2; // Skip next 2 words as overlapping only possible with shelf
            }
        } 
        i += 1;
    }
    (count_elf,count_elf_on_a_shelf,count_shelf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_counts() {
        let s = "In Belfast I heard an elf on a shelf on a shelf on a ";
        let (count_elf, count_elf_on_a_shelf,count_shelf) = count_words(s);
        assert_eq!(count_elf, 4);
        assert_eq!(count_elf_on_a_shelf, 2);
        assert_eq!(count_shelf, 0);

        let s = "Somewhere in Belfast under a shelf store but above the shelf realm there's an elf on a shelf on a shelf on a shelf on a elf on a shelf on a shelf on a shelf on a shelf on a elf on a elf on a elf on a shelf on a ";
        let (count_elf, count_elf_on_a_shelf,count_shelf)  = count_words(s);
        assert_eq!(count_elf, 16);
        assert_eq!(count_elf_on_a_shelf, 8);
        assert_eq!(count_shelf, 2);

        let s = "shelf shelf shelf shelf";
        let (count_elf, count_elf_on_a_shelf,count_shelf)  = count_words(s);
        assert_eq!(count_elf, 4);
        assert_eq!(count_elf_on_a_shelf, 0);
        assert_eq!(count_shelf, 4);
    }
}

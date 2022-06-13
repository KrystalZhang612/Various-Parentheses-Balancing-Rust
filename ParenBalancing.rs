//Algorithm:

//first push the opening brackets onto the string '
//then on the closing brackets, pop out the string and check if there is matches
//if not empty list at the end, indicating no matching, return false. 
//create a hashmap top correctly map the corresponding opening and closing brackets


//initialize boolean type function 

pub fn matching_parentheses(InputString: &str) -> bool {
	
	//create a new method to find matches named "s" 
	
	let mut s = Vec::new();
	
	//create a hashmap from a fixed-sized list of of opening and closing brackets pairs. 
	//to correctly map the corresponding opening and closing brackets while traversing the InputString 
	//map closing brackets onto their corresponding opening brackets. 
	
	let opening_brackets: std::collections::HashMap<char, char> =
	
		[('(', ')'), ('{', '}'), ('[', ']')]
		
		//iterate over the generic type
		
		//Creates an iterator which clones all of its char elements.
		
			.iter().cloned().map(|(c1, c2)| (c2, c1)).collect();
			

	//traverse the entire InputString 
	
	for myChar in InputString.chars() {
		
		match myChar {
			
			'(' | '{' | '[' => s.push(myChar),
			
			')' | '}' | ']' => match (s.pop(), opening_brackets.get(&myChar)) {
				
				(Some(opening_bracket_left), Some(&string_expression_with_opening_bracket_left)) => {
					
					//if after popping myChar, the left string is not an opening bracket character 
					//then we didn't pop any closing bracket, we poped on empty then. 
					//which indicates there is an unmatched opening bracket left, return false. 
					
					
					if opening_bracket_left != string_expression_with_opening_bracket_left {
						
						return false;
					}
					
				}
				
				//if there are still any opening/closing hash pair brackets left after popping 
				// then it is not empty at the end, and the leftovers must be unmatched. 
				//therefore return false as well.
				
				(_,_) => return false,
				
			},
			//if the the array is emnpty at the end
			//the string traversal is done, return true
			_ => (),
		}
	}
	
	//return empty array since we have already found all matches and determined the boolean result. 
	
	s.is_empty()
}


//driver 

fn main() {
	
	//really complicated to prompt user input in rust
	//so I preset input myString variables to achieve different test results. 
	
	
	let myString: &str = "  "; 
	
		
	if matching_parentheses(myString){
		
	
		println! ("Input: {}", myString); 
		
		println! ("Output: true"); 
	}
	
	else if !matching_parentheses(myString){
		
		println! ("Input: {}", myString);
		
		println! ("Output: false"); 
		
	} 	
}

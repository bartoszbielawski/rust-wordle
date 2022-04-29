use rand::seq::SliceRandom; 
use counter::Counter;
use std::iter::zip;
use termion::color;
use std::io;
use std::fs;
use std::collections::HashMap;


#[derive(PartialEq,Debug)]
enum LetterState
{
	Unknown,
	NotPresent,
	WrongPlace,
	RightPlace
}

const color_map: HashMap<char, termion::color::Color> = [
	(LetterState::Unknown, color::LightWhite),
	(LetterState::NotPresent, color::White),
	(LetterState::WrongPlace, color::LightYellow),
	(LetterState::RightPlace, color::LightGreen),
];

fn guess_word(hidden: &str, guess: &str, letter_states: &mut HashMap::<char, LetterState>) -> bool
{
	let mut hidden_counter = Counter::<char>::new();
	hidden_counter.update(hidden.chars());
	//println!("{:?}", hidden_counter);
	
	let mut all_found = true;
	
	if hidden.len() != guess.len()
	{
		println!("{} {}", hidden.len(), guess.len());
		return false;
	}
	
	for (ch1, ch2) in zip(guess.chars(), hidden.chars())
	{
		let chars_equal = ch1 == ch2;
		all_found = all_found & chars_equal;
		
		let letter_state = letter_states.entry(ch1).or_insert(LetterState::NotPresent);
		
		if chars_equal
		{
			print!("{}{}", color::Fg(color::LightGreen), ch1);
			hidden_counter.subtract(ch1.to_string().chars());		//remove one
			*letter_state = LetterState::RightPlace;
			continue;  
		}
		let letter_present = hidden_counter.contains_key(&ch1);
		if letter_present
		{
			print!("{}{}", color::Fg(color::LightYellow), ch1);
			//remove counter as well so that it doesn't show again
			//somewhere else
			hidden_counter.subtract(ch1.to_string().chars());
			if *letter_state != LetterState::RightPlace
			{
				*letter_state = LetterState::WrongPlace;
			}
			continue;
		}
		
		*letter_state = LetterState::NotPresent;
		print!("{}{}", color::Fg(color::Reset), ch1);
	}
	println!("{}", color::Fg(color::Reset));
	return all_found;
}


fn load_words(filename: &str) -> Option<Vec<String>>
{
	let data = fs::read_to_string(filename).expect("Couldn't read from file!");

	let words: Vec<String> = data.lines().
		filter(|x| x.len() == 5).
		filter(|x| x.chars().all(char::is_alphabetic)).
		map(|x|    x.to_string()).
		filter(|x| x.is_ascii()).
		collect();
	
	println!("Loaded {} words...", words.len());
	
	return Some(words);
}


fn main() 
{
	let words = load_words("dict.txt").unwrap();
	
	let mut rng = rand::thread_rng();
	let hidden = words.choose(&mut rng).unwrap();

	let mut letter_state = HashMap::new();
		
	for ch in 'a'..'z'
	{
		letter_state.insert(ch, LetterState::Unknown);
	}
	
	let mut guessed = false;
	
	for i in 1..6
	{
		println!("Try {}: ", i);		
		
		let guess = loop
		{
			let mut guess = String::new();

			io::stdin()
				.read_line(&mut guess)
				.expect("Failed to read line");
			
			guess = guess.trim().to_string();
			
			if guess.len() != 5
			{
				println!("We need exactly 5 letters...");
				continue;
			}
			
			if !words.contains(&guess)
			{
				println!("Word not found!");
				continue;
			}
			 
			break guess;
		};
		
		if guess_word(&hidden, guess.trim(), &mut letter_state)
		{
			println!("You have guessed the word!");
			guessed = true;
			break;
		}
		
		for ch in 'a'..'z'
		{
			
			print!("{} - {:?}", ch, letter_state.get(&ch).unwrap());
		}
	}
	
	if !guessed
	{
		println!("The word you've been trying to find is '{}'", hidden);
	}
	
}

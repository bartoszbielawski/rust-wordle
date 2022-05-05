use rand::seq::SliceRandom; 
use counter::Counter;
use std::iter::zip;
use termion::color;
use std::io;
use std::fs;
use std::collections::HashMap;
use colored::Colorize;

#[derive(PartialEq,Debug)]
enum LetterState
{
	Unknown,
	NotPresent,
	WrongPlace,
	RightPlace
}

enum GuessResult
{
	WrongInput,
	Try(GuessContent),
	Won(GuessContent),
	Lost(String),
}

struct Game
{
	word_list: Vec<String>,
	letter_states: HashMap<char, LetterState>,
	hidden_word: String,
	letter_counter: Counter::<char>,
	try_no: u32,
	finished: bool,
}

struct GuessContent
{
	try_no: u32,
	word_letter_states: Vec<LetterState>,
}


impl Game
{
	fn new(words: Vec<String>) -> Self
	{
		let mut game = Game {word_list: words, 
				letter_states: HashMap::<char, LetterState>::new(), 
				hidden_word: "".to_string(), 
				letter_counter: Counter::<char>::new(),
				try_no: 0, 
				finished: false};
			
		let mut rng = rand::thread_rng();
		
		game.hidden_word = game.word_list.choose(&mut rng).unwrap().to_string();  
		game.letter_counter.update(game.hidden_word.chars());
		
		return game;
	}
	
	fn word_valid(&self, word: &String) -> bool
	{
		if word.len() != 5
		{
			return false;
		}
		
		if !self.word_list.contains(&word)
		{
			return false;
		}
		
		return true;
	}
	
	fn guess_word(&mut self, guess: &String) -> GuessResult
	{
		if !self.word_valid(guess)
		{
			return GuessResult::WrongInput;
		}
			
		let mut letter_counter = self.letter_counter.clone();
		
		let mut wls = vec![(); 5]; 
		
		for i in 0..=4
		{
			println!("{}", i);
		}
		
		for (ch1, ch2) in zip(guess.chars(), self.hidden_word.chars())
		{
			let chars_equal = ch1 == ch2;
			if chars_equal
			{
				
			}
		}
		
		
		GuessResult::Lost(self.hidden_word)
	}
}


fn guess_word(hidden: &str, guess: &str, letter_states: &mut HashMap::<char, LetterState>) -> bool
{
	let mut hidden_counter = Counter::<char>::new();
	hidden_counter.update(hidden.chars());
	//println!("{:?}", hidden_counter);
	
	if hidden.len() != guess.len()
	{
		return false;
	}
	
	if guess == hidden
	{
		return true;
	}
	
	//a variable for state of all letters in the word
	//HERE
	
	for (ch1, ch2) in zip(guess.chars(), hidden.chars())
	{
		let chars_equal = ch1 == ch2;
		
		//all letters were already created
		let letter_state = letter_states.get_mut(&ch1).unwrap();
		if !chars_equal
		{
			continue;
		}
		
		print!("{}{}", color::Fg(color::LightGreen), ch1);
		hidden_counter.subtract(ch1.to_string().chars());		//remove one
		*letter_state = LetterState::RightPlace;
	}
	
	for (ch1, ch2) in zip(guess.chars(), hidden.chars())
	{
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
		
		if (*letter_state == LetterState::WrongPlace) || (*letter_state == LetterState::RightPlace)
		{
			*letter_state = LetterState::NotPresent;
		}
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
		map(|x|    x.to_lowercase()).
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
		
		print!("\t\t");
		for ch in 'a'..'z'
		{
			let ls = letter_state.get(&ch).unwrap();
			let color: colored::Color = match ls
			{
				LetterState::Unknown => colored::Color::White,
				LetterState::NotPresent => colored::Color::Black,
				LetterState::RightPlace => colored::Color::BrightGreen,
				LetterState::WrongPlace => colored::Color::BrightYellow,
			}; 
			print!("{}", ch.to_string().color(color));
		}
		println!();
	}
	
	if !guessed
	{
		println!("The word you've been trying to find is '{}'", hidden);
	}
	
}

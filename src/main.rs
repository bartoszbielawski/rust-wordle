use rand::seq::SliceRandom; 
use counter::Counter;
use std::iter::zip;

use std::io::{self, Write};
use std::fs;
use std::collections::HashMap;
use colored::Colorize;
use itertools::izip;
use std::fmt;

#[derive(PartialEq,Debug,Clone,Copy)]
enum LetterState
{
	Unknown,
	NotPresent,
	WrongPlace,
	RightPlace
}

#[derive(Debug)]
enum GuessResult
{
	UnknownWord,
	WrongInput,
	Try(Guess),
	Won(u32),
	Lost(String),
}

struct Game
{
	word_list: Vec<String>,
	letter_states: HashMap<char, LetterState>,
	hidden_word: String,
	letter_counter: Counter::<char>,
	try_no: u32,
}

#[derive(Debug)]
struct Guess
{
	try_no: u32,
	guess: String,
	word_letter_states: [LetterState; 5]
}


impl std::fmt::Display for Guess
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let mut s = write!(f, "Try {} => ", self.try_no);
		for (ch, state) in zip(self.guess.chars(), self.word_letter_states)
		{
			let color: colored::Color = match state
			{
				LetterState::Unknown => colored::Color::BrightWhite,
				LetterState::NotPresent => colored::Color::White,
				LetterState::RightPlace => colored::Color::BrightGreen,
				LetterState::WrongPlace => colored::Color::BrightYellow,
			};
			s = write!(f, "{}", ch.to_string().color(color));
		}
		return s;
	}
}



impl Game
{
	fn new(words: Vec<String>) -> Self
	{
		let mut rng = rand::thread_rng();
		let hidden_word = words.choose(&mut rng).unwrap().to_string();
		let hidden_word_chars = hidden_word.chars();
		let letter_counter = Counter::<char>::from_iter(hidden_word_chars);

		let mut letter_states = HashMap::<char, LetterState>::new();
		for ch in 'a'..='z'
		{
			letter_states.insert(ch, LetterState::Unknown);
		}


		let mut game = Game {
				word_list: words,
				letter_states,
				hidden_word,
				letter_counter,
				try_no: 1};

		return game;
	}
	
	fn word_valid(&self, word: &String) -> bool
	{
		if word.len() != 5
		{
			return false;
		}
		

		return true;
	}

	fn word_known(&self, word: &String) -> bool
	{
		if !self.word_list.contains(&word)
		{
			return false;
		}
		true
	}
	
	fn guess_word(&mut self, guess: &String) -> GuessResult
	{
		if !self.word_valid(guess)
		{
			return GuessResult::WrongInput;
		}

		if !self.word_known(guess)
		{
			return GuessResult::UnknownWord;
		}

		if self.hidden_word.eq(guess)
		{
			return GuessResult::Won(self.try_no);
		}

		let mut letter_counter = self.letter_counter.clone();
		let mut wls: [LetterState; 5] = [LetterState::NotPresent; 5];
		
		for (i, ch1, ch2) in izip!(0..=4, guess.chars(), self.hidden_word.chars())
		{
			if ch1 != ch2
			{
				continue;
			}

			//letters are equal
			wls[i] = LetterState::RightPlace;
			letter_counter.subtract(ch1.to_string().chars())
		}

		for (i, ch1) in zip(0..=4, guess.chars())
		{
			let letter_present = letter_counter.contains_key(&ch1);
			if !letter_present
			{
				continue;
			}

			wls[i] = LetterState::WrongPlace;
			//remove counter as well so that it doesn't show again
			//somewhere else
			letter_counter.subtract(ch1.to_string().chars());
		}

		if self.try_no >= 6
		{
			return GuessResult::Lost(self.hidden_word.clone());
		}

		let g = Guess {try_no: self.try_no, guess: guess.to_string(), word_letter_states: wls};

		self.try_no += 1;

		return GuessResult::Try(g);
	}
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
	
	let mut g = Game::new(words);

	while true
	{
		let guess = loop
		{
			print!("Try {} => ", g.try_no);
			io::stdout().flush();

			let mut guess = String::new();

			io::stdin()
				.read_line(&mut guess)
				.expect("Failed to read line");

			guess = guess.trim().to_string();
			break guess;
		};

		let r = g.guess_word(&guess);

		match r
		{
			GuessResult::UnknownWord => println!("The word is not known!"),
			GuessResult::Try(x) => println!("{}", x),
			GuessResult::WrongInput => println!("Wrong input!"),
			GuessResult::Lost(s) => {
				println!("The word you were trying to guess was {}", s.color(colored::Color::BrightGreen));
				break;},
			GuessResult::Won(_x) => {
				println!("You won!");
				break},
		};
	}
}

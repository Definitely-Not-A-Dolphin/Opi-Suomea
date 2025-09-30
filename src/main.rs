use rand::Rng;
use std::fs;
use std::fs::File;
use std::io::{self, Write};

struct Question {
  asked: String,
  answer: String,
  language: String,
}

fn div() {
  print!("\n\x1b[34m-+==================+-\x1b[0m\n\n");
}

fn count_lines(path_name: &String) -> i32 {
  let contents = fs::read_to_string(path_name).expect("Something went wrong!");

  let mut line_count: i32 = 1;
  for i in contents.chars() {
    if i == '\n' {
      line_count += 1;
    }
  }

  return line_count;
}

fn get_line(path_name: &String, line: i32) -> String {
  let contents = fs::read_to_string(path_name).expect("An error occurred!");
  let mut cur_line: i32 = 1;
  let mut return_value = String::new();

  for i in contents.chars() {
    if cur_line == line {
      return_value.push(i);
    }
    if i == '\n' {
      cur_line += 1;
    }
  }

  return return_value;
}

#[expect(dead_code)]
fn get_element(path_name: &String, row: i32, column: i32) -> String {
  let contents = get_line(path_name, row);

  let mut cur_column: i32 = 1;
  let mut cur_cell = String::new();

  for i in contents.chars() {
    if cur_column == column {
      cur_cell.push(i);
    }
    if i == ';' {
      cur_column += 1;
    }
  }

  return cur_cell.trim().to_string();
}

fn parse_to_vector(path_name: &String) -> Vec<Vec<String>> {
  let contents = fs::read_to_string(path_name).expect("An error occurred!");
  let mut full_vector: Vec<Vec<String>> = vec![];
  let mut row_vector: Vec<String> = vec![];
  let mut cur_cell = String::new();
  for i in contents.chars() {
    cur_cell.push(i);
    if i == ';' {
      cur_cell.pop();
      row_vector.push(cur_cell.clone());
      cur_cell.clear();
    }
    if i == '\n' {
      full_vector.push(row_vector.clone());
      row_vector.clear();
      cur_cell.clear();
    }
  }

  return full_vector;
}

fn parse_to_csv(path_name: &String, data_vector: &Vec<Vec<String>>) {
  let mut append_file = fs::OpenOptions::new().write(true).open(path_name).unwrap();
  for i in 0..data_vector.len() {
    for j in 0..data_vector[i].len() {
      let thing = data_vector[i][j].clone();
      if let Err(e) = write!(append_file, "{};", &thing[..]) {
        eprintln!("Couldn't write to file: {}", e);
      };
    }
    if let Err(e) = write!(append_file, "\n") {
      eprintln!("Couldn't write to file: {}", e);
    };
  }
}

fn get_cur_question_data(path_name: &String, line: i32) -> Question {
  let contents = get_line(path_name, line);
  let mut important_vector: Vec<String> = vec![];
  let mut cur_cell = String::new();
  for i in contents.chars() {
    if i != ';' {
      cur_cell.push(i);
      continue;
    }
    important_vector.push(cur_cell.clone().trim().to_string());
    cur_cell.clear();
  }
  return Question {
    asked: important_vector[0].clone(),
    answer: important_vector[1].clone(),
    language: important_vector[2].clone(),
  };
}

fn do_lesson(path_name: &String) -> f32 {
  let question_count: i32 = count_lines(path_name) - 1;

  let mut correct: i32 = 0;

  for i in 1..=question_count {
    let thing: Question = get_cur_question_data(path_name, i);

    div();

    print!("Translate to {}\n", thing.language);
    print!("{}: ", thing.asked);
    std::io::stdout().flush().unwrap();
    let mut given_answer = String::new();
    io::stdin()
      .read_line(&mut given_answer)
      .expect("Something went wrong getting your answer!");

    if given_answer.trim().to_string().to_lowercase() == thing.answer.to_lowercase() {
      print!("\nYippie! :D\n");
      correct += 1;
    } else {
      print!("\nOops :(, the correct word was \"{}\"\n", thing.answer);
    }
  }

  let grade = (correct as f32 / question_count as f32 * 100.0).floor() as f32 / 10.0;
  println!("Your grade is: {}/10", grade);

  for _ in 1..=3 {
    let random_int = rand::thread_rng().gen_range(1..question_count);
    let thing: Question = get_cur_question_data(path_name, random_int);

    div();
    print!("Recap\n");
    print!("Translate to {}\n{}: ", thing.language, thing.asked);

    std::io::stdout().flush().unwrap();
    let mut given_answer = String::new();
    io::stdin()
      .read_line(&mut given_answer)
      .expect("Something went wrong getting your answer!");

    if given_answer.trim().to_string().to_lowercase() == thing.answer.to_lowercase() {
      print!("\nYippie! :D\n");
    } else {
      print!("\nOops :(, the correct word was \"{}\"\n", thing.answer);
    }
  }

  return grade;
}

fn check_vocab(path_name: &String) {
  let vocab_vector = parse_to_vector(&path_name);
  print!("English: \x1b[31mFinnish\x1b[0m\n");
  for i in vocab_vector {
    if i[2].trim().to_string() == "Finnish" {
      let a = i[0].trim().to_string();
      let b = i[1].trim().to_string();
      println!("{}: \x1b[31m{}\x1b[0m", a, b);
    };
  }
}

fn main() {
  // Create
  match File::create_new("saveData.csv") {
    Ok(mut file) => {
      let mut lessons_temp =
        File::open("baseData.csv").expect("An error occured making your save data!");
      match io::copy(&mut lessons_temp, &mut file) {
        Ok(okiedokie) => {
          println!("{}", okiedokie);
        }
        Err(e) => {
          panic!("{}", e);
        }
      }
    }
    Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
      println!("Okay your file exists!")
    }
    Err(e) => {
      println!("An error occured while making your save data!");
      panic!("{}", e);
    }
  }

  // Get lessons
  let mut lessons_data = parse_to_vector(&String::from("saveData.csv"));

  loop {
    div();

    print!("Welcome to Opi Suomea! Please select the lesson you want to practise:\n");

    let activity;
    loop {
      print!("Please select what you would like to do:\n");
      print!("\t1. Do a lesson,\n");
      print!("\t2. Check vocab of a lesson,\n");
      print!("\t0. Save and quit\n");

      io::stdout().flush().unwrap();
      let mut input_1 = String::new();
      io::stdin()
        .read_line(&mut input_1)
        .expect("Oopsie, something went wrong!");
      input_1 = input_1.trim().to_string();
      if input_1 == "0" || input_1 == "1" || input_1 == "2" {
        activity = input_1;
        break;
      }
      print!("That is not a valid input. Please try again:\n");
    }

    div();

    if activity == "0" {
      parse_to_csv(&String::from("saveData.csv"), &lessons_data);
      print!("Bye bye!\n");
      break;
    }

    if activity == "1" {
      print!("Your lessons:\n");
      print!("0. Go back\n");
      for i in 0..lessons_data.len() {
        if lessons_data[i][3] != "0" {
          print!(
            "{}. {}\n\t{}\n\tCurrent best grade: {}/10\n",
            i + 1,
            lessons_data[i][1],
            lessons_data[i][2],
            lessons_data[i][4]
          );
        }
      }

      let number: i32 = loop {
        let mut lesson_input = String::new();
        io::stdin().read_line(&mut lesson_input).expect("Oopsie!");
        match lesson_input.trim().parse::<i32>() {
          Ok(n) => {
            if n == 0 {
              break 0;
            }
            if n < 1 || (lessons_data.len() as i32) < n || lessons_data[(n - 1) as usize][3] != "1"
            {
              println!("That is not a valid input! Please try again: ");
              continue;
            };
            break n;
          }
          Err(_err) => {
            println!("That is not a valid input! Please try again: ");
            continue;
          }
        }
      };

      if number == 0 {
        continue;
      }

      let mut path_name = String::from("lessons/");
      path_name.push_str(lessons_data[(number - 1) as usize][0].clone().as_str());
      path_name.push_str(".csv");
      print!("You have chosen lesson {}\n", lessons_data[(number - 1) as usize][1]);
      let new_grade = do_lesson(&path_name);
      let old_grade = lessons_data[(number - 1) as usize][4].clone();
      let old_grade_float: f32 = match old_grade.trim().parse::<f32>() {
        Ok(n) => n,
        Err(err) => {
          print!(
            "An error occured coverting a string into f32! Error: {}\n",
            err
          );
          0f32
        }
      };
      if new_grade > old_grade_float {
        print!("New best grade! {}/10\n", new_grade);
        lessons_data[(number - 1) as usize][4] = new_grade.to_string();
        parse_to_csv(&path_name, &lessons_data);
      } else {
        print!("Your grade: {}/10\n", new_grade);
      }
      // Give access to next lesson
      if lessons_data.len() > number as usize {
        lessons_data[number as usize][3] = String::from("1");
      }
    }

    if activity == "2" {
      print!("Look up the words!\n");
      print!("0. Go back\n");
      for i in 0..lessons_data.len() {
        if lessons_data[i][3] != "0" {
          print!(
            "{}. {}\n\t{}\n\tCurrent best grade: {}/10\n",
            i + 1,
            lessons_data[i][1],
            lessons_data[i][2],
            lessons_data[i][4]
          );
        }
      }

      let number: i32 = loop {
        let mut lesson_input = String::new();
        io::stdin().read_line(&mut lesson_input).expect("Oopsie!");
        match lesson_input.trim().parse::<i32>() {
          Ok(n) => {
            if n == 0 {
              break 0;
            };
            if n < 1 || (lessons_data.len() as i32) < n || lessons_data[(n - 1) as usize][3] != "1"
            {
              print!("That is not a valid input! Please try again:\n");
              continue;
            };
            break n;
          }
          Err(_err) => {
            print!("That is not a valid input! Please try again:\n");
            continue;
          }
        }
      };

      let mut path_name = String::from("lessons/");
      path_name.push_str(lessons_data[(number - 1) as usize][0].clone().as_str());
      path_name.push_str(".csv");
      print!(
        "You have chosen lesson {}\n",
        lessons_data[(number - 1) as usize][1]
      );

      div();

      check_vocab(&path_name);
    }
  }
}

use anyhow::Result;
use device_query::{DeviceQuery, DeviceState, Keycode};
use inquire::Text;
use enigo::{Enigo, KeyboardControllable, MouseControllable};
use rand::Rng;
use std::{thread, time};
use std::time::Duration;


#[derive(Debug)]
struct SearchParams {
    area_code: String,
    prefix: Option<String>,
}

fn main() -> Result<()> {
    println!("\"Fuck doing this shit by hand\" - Jason Hall");
    let device_state = DeviceState::new();
    let mut phone_numbers: Vec<String> = Vec::new();
    let mut enigo = Enigo::new();

    loop {
    // click back on terminal for input
    // ********************************
    // ***** COORD ********************
    // ********************************
    let terminal_location = (1003, 894);
    let randomized_location1 = randomize_coords(terminal_location);
    enigo.mouse_move_to(randomized_location1.0, randomized_location1.1);
    thread::sleep(random_delay());
    enigo.mouse_click(enigo::MouseButton::Left);

    // Prompt for area code (once)
    println!("\nOptions: [Enter 3-digit area code] or [Type p] or [Type d]");
        let input = Text::new(">").prompt()?;

        match input.trim() {
            "p" => {
                println!("\nStored numbers:");
                if phone_numbers.is_empty() {
                    println!("No numbers stored yet!");
                } else {
                    phone_numbers.iter().for_each(|n| println!("{}", n));
                }
                continue;
            }
            "d" => {
                phone_numbers.clear();
                println!("\nAll numbers deleted!");
                continue;
            }
            code if code.len() == 3 && code.chars().all(char::is_numeric) => {
                let prefix = Text::new("Enter the second number group (e.g. 555) or press Enter to skip:")
                    .prompt_skippable()?;

                let search_params = SearchParams {
                    area_code: code.to_string(),
                    prefix,
                };

        println!("\n Using Search Parameters: {:?}", search_params);

        

        // Placeholder coordinates (update with real ones)
        // ********************************
        // ***** COORD ********************
        // ********************************
        let area_code_coords = (2532, 199);
        let prefix_coords = (2793, 200);

        let randomized_location2 = randomize_coords(area_code_coords);
        let randomized_location3 = randomize_coords(prefix_coords);
        // Input area code
        enigo.mouse_move_to(randomized_location2.0, randomized_location2.1);
        thread::sleep(random_delay());
        enigo.mouse_click(enigo::MouseButton::Left);
        thread::sleep(random_delay());
        enigo.key_sequence(&search_params.area_code);

        // Input prefix if exists
        if let Some(prefix) = &search_params.prefix {
            enigo.mouse_move_to(randomized_location3.0, randomized_location3.1);
            thread::sleep(random_delay());
            enigo.mouse_click(enigo::MouseButton::Left);
            thread::sleep(random_delay());
            enigo.key_sequence(prefix);
        }

        // Press Enter
        thread::sleep(random_delay());
        enigo.key_click(enigo::Key::Return);

        // Ask user to verify
       
        println!("Does this number look okay (Y/N)");
        // Click out of input
        // ********************************
        // ***** COORD ********************
        // ********************************
        let random_location = (2793, 333);
        let randomized_location4 = randomize_coords(random_location);

        enigo.mouse_move_to(randomized_location4.0, randomized_location4.1);
        thread::sleep(random_delay());
        enigo.mouse_click(enigo::MouseButton::Left);

        if wait_for_yn_confirmation(&device_state) {
            thread::sleep(random_delay());
            let checkbox = (2063, 444);
            enigo.mouse_move_to(checkbox.0, checkbox.1);
            thread::sleep(random_delay());
            enigo.mouse_click(enigo::MouseButton::Left);
            

            // Get phone number
            // Placeholder coordinates - replace these with actual values later
            // ********************************
            // ***** COORD ********************
            // ********************************
            let start_coords = (2183, 439);
            let end_coords = (2312, 441);

            let randomized_location5 = randomize_coords_y(start_coords);
            let randomized_location6 = randomize_coords_y(end_coords);

            // Move to start position and click-drag to select text
            enigo.mouse_move_to(randomized_location5.0, randomized_location5.1);
            thread::sleep(random_delay());
            enigo.mouse_down(enigo::MouseButton::Left);
            thread::sleep(random_delay());
            enigo.mouse_move_to(randomized_location6.0, randomized_location6.1);
            thread::sleep(random_delay());
            enigo.mouse_up(enigo::MouseButton::Left);

            // Copy selected text (Ctrl+C)
            thread::sleep(random_delay());
            enigo.key_down(enigo::Key::Control);
            enigo.key_click(enigo::Key::Layout('c'));
            enigo.key_up(enigo::Key::Control);

            // Get clipboard content
            thread::sleep(random_delay()); // Give time for clipboard to update
            let clipboard = arboard::Clipboard::new().unwrap().get_text().unwrap_or_default();

            // Clean the phone number
            let cleaned_number = clipboard
                .chars()
                .filter(|c| c.is_ascii_digit())
                .skip(1) // Skip the '1' in +1
                .take(10) // Take exactly 10 digits
                .collect::<String>();

            println!("Extracted phone number: {}", cleaned_number);
            phone_numbers.push(cleaned_number);

            // TODO purchase number button click && make prompts listen for a enter press outside of the terminal
            // ********************************
            // ***** COORD ********************
            // ********************************
            let button_one = (3352, 446);
            let randomized_location7 = randomize_coords(button_one);

            enigo.mouse_move_to(randomized_location7.0, randomized_location7.1);
            thread::sleep(random_delay());

            if rand::random() {
                // Double click
                enigo.mouse_click(enigo::MouseButton::Left);
                thread::sleep(random_delay());
                enigo.mouse_click(enigo::MouseButton::Left);
            } else {
                // Triple click
                enigo.mouse_click(enigo::MouseButton::Left);
                thread::sleep(random_delay());
                enigo.mouse_click(enigo::MouseButton::Left); 
                thread::sleep(random_delay());
                enigo.mouse_click(enigo::MouseButton::Left);
            }


            // Wait for user to press Enter after page loads
            println!("New page loaded. Press Enter to continue...");
            wait_for_key(&device_state, Keycode::Enter);

            // Placeholder coordinates for the dropdown (update later)
            // ********************************
            // ***** COORD ********************
            // ********************************
            let dropdown_coords = (2457, 340);
            let randomized_location8 = randomize_coords(dropdown_coords);

            enigo.mouse_move_to(randomized_location8.0, randomized_location8.1);
            thread::sleep(random_delay());
            enigo.mouse_click(enigo::MouseButton::Left);

            // ********************************
            // ***** COORD ********************
            // ********************************
            let server_choice_coords = (2439, 428);
            let randomized_location9 = randomize_coords(server_choice_coords);

            enigo.mouse_move_to(randomized_location9.0, randomized_location9.1);
            thread::sleep(random_delay());
            enigo.mouse_click(enigo::MouseButton::Left);


            thread::sleep(random_delay());
             // Placeholder coordinates for the button click (you'll update these later)
             // ********************************
            // ***** COORD ********************
            // ********************************
            let button_two = (3334, 343);
            let randomized_location10 = randomize_coords(button_two);

            enigo.mouse_move_to(randomized_location10.0, randomized_location10.1);
            thread::sleep(random_delay());
            enigo.mouse_click(enigo::MouseButton::Left);
            
            // Selecting an option from the dropdown (let's assume you need the option with a specific IP)
            // Here, we're simulating the selection of the first option
            // You’ll update this part with specific dropdown interaction logic later
            thread::sleep(random_delay());
            enigo.key_sequence("1"); // Assume IP selection corresponds to '1'

            println!("Next");
            thread::sleep(Duration::from_secs(1));
        } else {
            println!("\nOkay, let's try a different prefix.");
        }
    }
    _ => println!("Invalid input. Please enter a 3-digit area code, 'p', or 'd'."),
}
}
}

fn random_delay() -> time::Duration {
    let millis = rand::thread_rng().gen_range(150..600);
    time::Duration::from_millis(millis)
}

fn wait_for_yn_confirmation(device_state: &DeviceState) -> bool {
    loop {
        let keys = device_state.get_keys();
        if keys.contains(&Keycode::Y) {
            // Wait for key release
            while device_state.get_keys().contains(&Keycode::Y) {
                thread::sleep(Duration::from_millis(50));
            }
            return true;
        }
        if keys.contains(&Keycode::N) {
            // Wait for key release
            while device_state.get_keys().contains(&Keycode::N) {
                thread::sleep(Duration::from_millis(50));
            }
            return false;
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn wait_for_key(device_state: &DeviceState, key: Keycode) {

    loop {
        let keys = device_state.get_keys();
        if keys.contains(&key) {
            while device_state.get_keys().contains(&key) {
                thread::sleep(Duration::from_millis(50));
            }
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn randomize_coords((x, y): (i32, i32)) -> (i32, i32) {
    let mut rng = rand::thread_rng();

    // Calculate 1% of each coordinate for the maximum variance
    let x_variance = (x as f32 * 0.01).abs() as i32;
    let y_variance = (y as f32 * 0.01).abs() as i32;
    
    // Ensure we have at least 1 pixel variance for small coordinates
    let x_variance = x_variance.max(1);
    let y_variance = y_variance.max(1);
    
    // Generate random offsets within ±1% range
    let x_offset = rng.gen_range(-x_variance..=x_variance);
    let y_offset = rng.gen_range(-y_variance..=y_variance);
    
    // Apply offsets to original coordinates
    (x + x_offset, y + y_offset)
}

fn randomize_coords_y((x, y): (i32, i32)) -> (i32, i32) {
    let mut rng = rand::thread_rng();

    // Calculate 1% of each coordinate for the maximum variance
    let y_variance = (y as f32 * 0.01).abs() as i32;
    
    // Ensure we have at least 1 pixel variance for small coordinates
    let y_variance = y_variance.max(1);
    
    // Generate random offsets within ±1% range
    let y_offset = rng.gen_range(-y_variance..=y_variance);
    
    // Apply offsets to original coordinates
    (x, y + y_offset)
}

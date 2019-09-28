
//This program was created while following an algorithm found here: https://www.youtube.com/watch?v=RxTfc4JLYKs&list=PLRqwX-V7Uu6bJM3VgzjNV5YxVxUwzALHV&index=2

extern crate rand;

use rand::thread_rng;
use rand::Rng;

const WORD_LENGTH: usize = 30;
const POPULATION: usize = 10000;
const WORD: [&str; WORD_LENGTH] = ["T", "O", "B", "E", "O", "R", "N", "O", "T", "T", "O", "B", "E", "T", "H", "A", "T", "I", "S", "T", "H", "E", "Q", "U", "E", "S", "T", "I", "O", "N"];

fn main() {
    let mut rng = thread_rng();
    //create an N-list of elements
    let alphabet = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
    let mut phrases = Vec::new();
    for i in 0..POPULATION {
        phrases.push(vec![]);
        for j in 0..WORD_LENGTH {
            let value = alphabet[rng.gen_range(0, 26)];
            phrases[i].push(value);
        }
    }
    let mut word = 0;
    //Repeat It have evolved enough
    let mut cont = true;
    while cont {
        //calculate fitness of current generation
        let mut fitness_list = fitness(&phrases);
        let mut parent = Vec::new();
        let mut children = Vec::new();
        let mut confirmed = false;
        //iterate through all the phrases and run the % chance that they are chosen as a parent
        let mut rng = thread_rng();
        while children.len() < POPULATION || children.len() == 0 {
            for i in 0..fitness_list.len() {       //this line should be turned into a while loop, but im lazy so il just slap a while loop on top of the for loop so i dont have to change anything
                //if fitness_list[i] == 1.0 || confirmed == true {
                  //  if phrases[i] == WORD {
                    //    cont = false;
                      //  if confirmed != true {
                        //    word = i;
                      //  }
                    //    confirmed = true;
                  //  }
                //}

                if rng.gen_range(0f32, 1f32) < fitness_list[i] {
                    //if in here then phrase was selected as a parent
                    parent.push(&phrases[i]);
                }
                if parent.len() == 2 {
                    //there are now two parents so the gene for the new child can be randomized
                    //each parent needs to create two children
                    let mut child1 = vec![]; // |
                    let mut child2 = vec![]; // |
                    let mut value1 = 0;      // ---> all of this should be added into one struct
                    let mut value2 = 1;     // |
                    for i in 0..WORD_LENGTH {
                        //random gene mutation chance (0.1%)
                        if rng.gen_range(0, 1000) == 1 {
                            child1.push(alphabet[rng.gen_range(0, 26)]);
                            child2.push(parent[value1][i]);
                        }
                        else if rng.gen_range(0, 1000) == 1 {
                            child2.push(alphabet[rng.gen_range(0, 26)]);
                            child1.push(parent[value1][i]);
                        }
                        else {
                            child1.push(parent[value1][i]);
                            child2.push(parent[value2][i]);
                        }
                        value2 = value1;
                        if value1 == 0 {    
                            value1 = 1;
                        }
                        else {
                            value1 = 0;
                        }
                    }
                    children.push(child1);
                    children.push(child2);
                    //dispose of parents in list once two children are created
                    parent.pop();
                    parent.pop();
                }
            }
        }
        phrases = children;
        fitness_list = fitness(&phrases);
        for i in 0..fitness_list.len() {
            if fitness_list[i] == 1.0 || confirmed == true {
                if phrases[i] == WORD {
                    cont = false;
                    word = i;
                }
            }
        }
        println!("{:?}", phrases[find_max(&fitness_list)]);
        
    }
    println!("{:?}", phrases[word]);
}

fn fitness(phrases: &Vec<Vec<&str>>) -> Vec<f32> {
    //Calculate fitness based of how many words of the letter we want each phrase has
    let mut fitness_list = Vec::new();
    for i in 0..POPULATION {
        let mut fitness = 0;
        for j in 0..WORD_LENGTH {
            if phrases[i][j] == WORD[j] {
                fitness += 1;
            }
        }
        let push_value = fitness as f32 / WORD_LENGTH as f32;
        fitness_list.push(push_value);
    }

    return fitness_list
}

fn find_max(vector: &Vec<f32>) -> usize {
    let mut largest = 0f32;
    let mut index = 0;
    for i in 0..vector.len() {
        if vector[i] > largest {
            largest = vector[i];
            index = i;
        }
    }
    index
}
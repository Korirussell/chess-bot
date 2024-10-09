// i think i'm going to have to make my own crates - engine and utils
// calling your own local crates goes like ex. crate::engine::*; or crate::utils::*; 
// also going to have to create a ZobristHasher crate, I really do not know what this is yet
use colored::*; // used to format text with colors for terminal output 
use std::fmt; // used for formating 
use std::str:FromStr; //used for converting strings into custom types (e.g., parsing chess moves from strings)

#[derive(Copy, Clone, PartialEq, Eq, Debug)] // the derive attribute in rust auto implements certain traits for a struct or enum.
/*
Explaining the attributes for derive because IDK
Copy: This allows your enum or struct to get copied 
Clone: This allows for you object to be deep copied ( types that own heap memory and stuff) , you can call it with ex. exobject.clone();
PartialEq: Allows for comparison of two values using == and !=, use this if you wana compare instances of your type
Eq: Uhhh requires if a == b is true then b == a you know, do not need to implement Eq manually 
Debug: Allows for fomating a value using hte {:?} formatter in println!, use if you want to print out your types value easily, useful for debuggin process duh
*/
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King, 
}

#[derive(Copy, Clone, Debug)]
enum Color {
    white,
    black,
}

#[derive(Copy, CLone, Debug)]
struct piece { 
    piece_type: PieceType,
    color: Color,
}
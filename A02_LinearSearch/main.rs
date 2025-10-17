/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: hirwatan <hirwatan@student.42tokyo.jp>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/10/17 20:24:57 by hirwatan          #+#    #+#             */
/*   Updated: 2025/10/17 21:05:05 by hirwatan         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io;
use std::str;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("ERROR"); //改行で受け取る
    let parse: Vec<&str> = input.trim().split(' ').collect();
    
    let _n: i32 = parse[0].parse().expect("ERROR");
    let x: i32 = parse[1].parse().expect("ERROR");
    
    let mut second_input = String::new();
    io::stdin().read_line(&mut second_input).expect("ERROR");
    let nums: Vec<&str> = second_input.trim().split(' ').collect();

    if nums.iter().any(|s|{
        s.parse::<i32>().map_or(false,|num| num == x)
    }) {
        println!("Yes");
    }  else {
        println!("No");
    }
}

/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: hirwatan <hirwatan@student.42tokyo.jp>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/10/19 12:21:38 by hirwatan          #+#    #+#             */
/*   Updated: 2025/10/19 13:36:53 by hirwatan         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("ERROR");
    let trim_input = input.trim();

    match i32::from_str_radix(trim_input,10){
        Ok(num) => {
            println!("{:010b}",num);
        }
        Err(_) => {
            eprintln!("ERROR");
        }
    }
}

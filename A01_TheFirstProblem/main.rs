/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: hirwatan <hirwatan@student.42tokyo.jp>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/10/16 23:46:10 by hirwatan          #+#    #+#             */
/*   Updated: 2025/10/17 00:27:14 by hirwatan         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io;

fn square_calculation(n: i32) -> i32 {
    return n * n;
}


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("REASON");
    let n: i32 = input.trim().parse().expect("REASON");
    let area = square_calculation(n);
    println!("{}",area);
}
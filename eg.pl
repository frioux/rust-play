#!/usr/bin/env perl

use strict;
use warnings;

use FFI::Platypus;
my $ffi = FFI::Platypus->new;
# $ffi->lang('Rust');
$ffi->lib('./target/debug/libembed.so');

$ffi->attach( 'add' => [qw(int int)] => 'int' );

print add(1, 2);

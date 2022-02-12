# Integrating with iOS

## Approach A: Without Cocoapods

*Credit to [brotskydotcom/rust-on-ios](https://github.com/brotskydotcom/rust-on-ios)
where this method is derived from.*

Setting up `flutter run` for iOS is somewhat more complicated than other platforms,
due to its reliance on the XCode user interface. This guide assumes you are running
a relatively recent version of XCode, which at the time of writing is XCode 13.
Other versions might have minor variances but the overall process should be the same.

In the following subsections, we will discuss how to achieve it without Cocoapods.

## Approach B: With Cocoapods

If you are using Cocoapods (Flutter apps by default use it), you can also integrate 
using it. Roughly speaking, follow the other approach, but when you need to set up
scripts (add build phases), add something like the following in your `.podspec`,
and Cocoapod will automatically help you inject the script phase when you execute
it as normal (`pod install`).

```ruby
  s.script_phase = {
      :name => 'Compile Rust',
      :script => ...somehow execute your script...,
      :execution_position => :before_compile,
      :shell_path => '/bin/sh'
   }
```

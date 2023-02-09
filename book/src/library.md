# Creating a Dart/Flutter library
In this chapter, we discuss how to add `flutter_rust_bridge` (FRB)
to an already existing application or create a new application from scratch;
this section covers creating a Dart-only library with a Flutter wrapper library on top.

In many cases, following this guide is actually easier long-term than creating
an application around FRB directly, but it does have a bit of overhead to set up.
This is also true when only using a library internally, because this guide will
also help you get set up with Melos, a monorepo tool specifically built for Dart/Flutter.

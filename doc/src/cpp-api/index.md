# C++ API

The C++ API allows you to use DQCsim in a more abstract way than the C API can
provide, by making use of C++11 features. The C++ API is a header-only wrapper
around the C API, so the two APIs use the same shared object file. The C++
headers are automatically installed along with the DQCsim Python package (more
detailed notes [here](../install/index.html)).

## How to read this chapter

This chapter provides basic information about the C++ API, as well as a few
examples to get you coding quickly. It is assumed that you already know what
DQCsim is, and have a decent understanding of the basic concepts. If you don't,
start [here](../index.md).

However, this chapter doesn't even get close to documenting every single
feature. If you're looking for something more complete, check out the generated
API documentation [here](../cpp_/index.html). It is also advised to skim
through the [C API documentation](../c-api/index.md); the C++ API borrows
heavily from it (as it is in fact merely a wrapper around it), and its
documentation is much more complete.

## Contents

 - [Usage](usage.md)
 - [Comparison to the C API](ccompare.md)
 - [Plugin anatomy](plugin.md)
 - [Host/simulation anatomy](sim.md)
 - [Reference](reference.md)

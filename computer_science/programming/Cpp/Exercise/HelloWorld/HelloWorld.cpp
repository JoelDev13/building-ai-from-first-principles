#include <iostream>
#include <string>
#include <memory>

int main() {
auto str = std::make_unique<std::string>("Hello World");
std::cout << *str << std::endl;
return 0;
}


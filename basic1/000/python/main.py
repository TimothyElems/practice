print("Hello, World!")


def main():
    hello = "Hello"
    world = "World!"

    print(hello,",", world) # This prints with a space between each concatenation
    print(hello + ", " + world, end="") # This adds a percent sign at the end
    print(hello + ", " + world)

if __name__ == "__main__":
    main()
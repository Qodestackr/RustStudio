defmodule MyApp do
  def run_demo do
    root_directory = FileSystem.create_directory("root")

    file1 = FileSystem.create_file("file1.txt", "Hello, World!")
    file2 = FileSystem.create_file("file2.txt", "Goodbye!")
    directory1 = FileSystem.create_directory("dir1")
    directory2 = FileSystem.create_directory("dir2")

    directory1 = FileSystem.add_item(directory1, file1)
    directory2 = FileSystem.add_item(directory2, file2)

    root_directory = FileSystem.add_item(root_directory, directory1)
    root_directory = FileSystem.add_item(root_directory, directory2)

    IO.inspect(FileSystem.list_items(root_directory))

    file1 = FileSystem.find_item(root_directory, "file1.txt")
    IO.inspect(file1)

    root_directory = FileSystem.delete_item(root_directory, "dir1")
    IO.inspect(FileSystem.list_items(root_directory))
  end
end

MyApp.run_demo()

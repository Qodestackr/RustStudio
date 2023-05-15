defmodule FileSystem do
  defstruct name: nil, content: nil, children: []

  def create_file(name, content) do
    %__MODULE__{name: name, content: content}
  end

  def create_directory(name) do
    %__MODULE__{name: name, children: []}
  end

  def add_item(directory, item) do
    %{directory | children: directory.children ++ [item]}
  end

  def list_items(directory) do
    directory.children
  end

  def find_item(directory, name) do
    Enum.find(directory.children, fn item -> item.name == name end)
  end

  def delete_item(directory, name) do
    %{directory | children: Enum.filter(directory.children, fn item -> item.name != name end)}
  end
end

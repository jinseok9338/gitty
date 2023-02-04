# Gitty

Gitty is a simple git CLI tool that allows you to 
1. clone the project and pull branches you need out of the gate.
2. sync with local branches with remote branches that are fetched --all --prune.
3. delete the branches you don't need anymore.

## Installation

Gitty is standard rust project. You can install it with cargo.

```bash

cargo build --release

```


## Usage
### you can can clone the repo to the directory you desire and pull the branches that you can choose from the command prompt
https://user-images.githubusercontent.com/27854958/216049428-6000dbad-23d5-4be3-9d2b-1e4b9d415713.mov

### you can sync with the local branches with remote branches that are fetched with prune, so it will always have the freshest remote branches
https://user-images.githubusercontent.com/27854958/216049920-e559cb46-87c3-42c3-b81a-44cc23be20af.mov


### you can delete local branches that you don't need anymore
https://user-images.githubusercontent.com/27854958/216050925-db882257-f6d2-4793-8573-f46c06d72b45.mov

###TODO
1. should clone private repo with auth provided [done]
2. error handling when pulling the repo that conflicts with remote
3. better color theme and welcome banner
4. need testing
5. make config file so you can fine tune how the app works (like auth)


## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)

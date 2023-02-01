# Gitty

Gitty is a simple git CLI tool that allows you to create a new repository on GitHub and sync it with your local machine.

## Installation

Gitty is standard rust project. You can install it with cargo.

```bash

cargo build --release

```


## Usage
### you can can clone the repo to the directory you desire and pull the branches that you can choose from the command prompt

```

```
### you can omit the directory option and it will clone the repo to the current directory
gitty up --url https://github.com/jinseok9338/xxxxx.git 
```

```
#if you don't provide the url option, it will sync local branches with remote branches of the directory(current directory if you don't provide the directory option) 
gitty up
```

```
#TODO
gitty --url https://github.com/jinseok9338/xxxxx.git --directory .
```

```
#TODO
gitty --url https://github.com/jinseok9338/xxxxx.git --directory .
```


## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
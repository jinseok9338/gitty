# Gitty

Gitty is a simple git CLI tool that allows you to create a new repository on GitHub and sync it with your local machine.

## Installation

todo

## Usage

```
#cloning the repo to your local machine with directory provided and sync it with remote branches
gitty up --url https://github.com/jinseok9338/xxxxx.git --directory .
```
```
#you can omit the directory option and it will clone the repo to the current directory
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
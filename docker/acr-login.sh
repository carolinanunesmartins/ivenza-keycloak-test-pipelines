if [ -z "$1" ]
  then
    echo "No argument supplied"
fi
az acr login --name $1 --expose-token > output
accessToken="$(grep -Po 'ey([^\"]*)' output)"
registry="$(grep -Po '\w*.azurecr\.io' output)"
docker login $registry -u 00000000-0000-0000-0000-000000000000 -p $accessToken
rm output

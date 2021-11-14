#!/bin/sh

function usage() {
    
}

role=""
playbook=""

while [[ $# -gt 0 ]]; do
  arg="$1"

  case $arg in
    -n | --name)
      role="$2"
      shift # past argument
      shift # past value
      ;;
    -p | --playbook)
      playbook="$2"
      shift
      shift
      ;;
    *)    # unknown option
      POSITIONAL+=("$1") # save it in an array for later
      shift # past argument
      ;;
  esac
done

if [ -z $role ];then
  echo "Must supply -n|--name for the role name"
  exit -1
fi

if [ -z $role ];then
  echo "Must supply -p|--playbook for the playbook name"
  exit -1
fi

if [ -e $playbook ];then
    if [ -e $role ];then
      echo "$role already exists"
      exit -1
    else 
      mkdir -p "$playbook/roles/$role"
      cp -r role-template/ "$playbook/roles/$role"
    fi
fi

tree "$playbook"
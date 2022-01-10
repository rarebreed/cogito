#!/bin/sh -xe

role=""
playbook=""

while [ $# -gt 0 ]; do
  arg="$1"

  case $arg in
    -n | --name)
      role="$2"
      shift
      shift
      ;;
    -p | --playbook)
      playbook="$2"
      shift
      shift
      ;;
    *)    # unknown option
      echo "Unknown arg"
      shift # past argument
      ;;
  esac
done

echo "Creating role $role in $playbook"

if [ -z $role ]; then
  echo "Must supply -n|--name for the role name"
  exit -1
fi

if [ -z $role ]; then
  echo "Must supply -p|--playbook for the playbook name"
  exit -1
fi

if [ ! -f $playbook ]; then
  mkdir $playbook
fi

if [ -f $role ]; then
  echo "$role already exists"
  exit -1
else 
  mkdir -p "$playbook/roles/$role"
  cp -r role-template/common/* "$playbook/roles/$role"
fi

if [ ! -f "$playbook/$playbook.yml" ]; then
  touch "$playbook/$playbook.yml"
fi

if [ ! -f "$playbook/inventory" ]; then
  mkdir "$playbook/inventory"
  touch "$playbook/inventory/cluster"
fi

tree "$playbook"
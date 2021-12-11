owd=`pwd`
cd serverless-fe2o3
cur=`pwd`
echo "in $cur"
echo "unlinking serverless-fe2o3"
npm unlink serverless-fe2o3
echo "linking"
npm link
cd "$owd"
cd lambda-example
cur=`pwd`
echo "in ${cur}"
SLS_DEBUG=* npx sls package
cd "$owd"
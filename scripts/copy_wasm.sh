mkdir -p wasms

export PACKAGE_NAME="user_index"
cp backend/canisters/$PACKAGE_NAME/api/can.did wasms/$PACKAGE_NAME.did

export PACKAGE_NAME="local_user_index"
cp backend/canisters/$PACKAGE_NAME/api/can.did wasms/$PACKAGE_NAME.did

export PACKAGE_NAME="post_index"
cp backend/canisters/$PACKAGE_NAME/api/can.did wasms/$PACKAGE_NAME.did

export PACKAGE_NAME="local_post_index"
cp backend/canisters/$PACKAGE_NAME/api/can.did wasms/$PACKAGE_NAME.did

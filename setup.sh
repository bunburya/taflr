# Android emulation
export JAVA_HOME="$HOME/.local/share/JetBrains/Toolbox/apps/android-studio/jbr"
export ANDROID_HOME="$HOME/Android/Sdk"
unset ANDROID_SDK_ROOT
export NDK_HOME="$ANDROID_HOME/ndk/29.0.13846066"
export PATH="$PATH:$ANDROID_HOME/emulator:$ANDROID_HOME/platform-tools"

# SQLX query checking
rm ./taflr.sqlite
sqlite3 ./taflr.sqlite < sql/schema.sqlite
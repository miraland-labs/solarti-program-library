#
# This file maintains the miraland versions for use by CI.
#
# Obtain the environment variables without any automatic updating:
#   $ source ci/miraland-version.sh
#
# Obtain the environment variables and install update:
#   $ source ci/miraland-version.sh install

# Then to access the miraland version:
#   $ echo "$miraland_version"
#

if [[ -n $MIRALAND_VERSION ]]; then
  miraland_version="$MIRALAND_VERSION"
else
  miraland_version=v1.18.2
fi

export miraland_version="$miraland_version"
export PATH="$HOME"/.local/share/miraland/install/active_release/bin:"$PATH"

if [[ -n $1 ]]; then
  case $1 in
  install)
    sh -c "$(curl -sSfL https://release.miraland.top/$miraland_version/install)"
    miraland --version
    ;;
  *)
    echo "miraland-version.sh: Note: ignoring unknown argument: $1" >&2
    ;;
  esac
fi

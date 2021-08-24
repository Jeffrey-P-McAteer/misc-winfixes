
import os
import sys
import subprocess


if __name__ == '__main__':
  subprocess.run([
    'cargo', 'build', '--release', '--target=x86_64-pc-windows-gnu',
  ], check=True)

  print('D/L and test with:')
  print('Invoke-WebRequest -Uri "http://localip.jmcateer.pw:8000/misc-winfixes.exe" -OutFile .\\misc-winfixes.exe ; .\\misc-winfixes.exe')
  subprocess.run([
    sys.executable, '-m', 'http.server',
  ], cwd=os.path.join('target', 'x86_64-pc-windows-gnu', 'release'), check=True)


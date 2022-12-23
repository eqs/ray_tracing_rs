import os
import click
from PIL import Image


@click.command()
@click.argument('path')
def cmd(path):
    img = Image.open(path)
    root, ext = os.path.splitext(path)
    img.save(root + '.png')


if __name__ == '__main__':
    cmd()

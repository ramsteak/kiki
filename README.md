# Kiki - Steganography CLI Tool

Kiki is a command-line application designed for hiding text or data within images using various steganographic methods.

## Features

- **Embed Data**: Hide text or binary data in images.
- **Extract Data**: Retrieve hidden data from images.
- **Methods Supported**: Currently, supports Least Significant Bit (LSB) embedding.

## Usage

Run the application with one of the following commands:

- `embed`: To hide data within an image.
- `extract`: To retrieve hidden data from an image.

For detailed command usage and options, run `kiki.exe help` or refer to the specific command’s help.

## Example Commands

- **Embed Data**: `kiki.exe embed input.png output.png secret.txt -m LSB -k mykey`
- **Extract Data**: `kiki.exe extract input.png output.txt  -m LSB -k mykey`
- **Extract Data to console**: `kiki.exe extract input.png - -m LSB -k mykey`

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

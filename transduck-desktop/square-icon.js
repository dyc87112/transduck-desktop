import sharp from 'sharp';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const inputPath = path.join(__dirname, 'src/assets/transduck-icon.png');
const outputPath = path.join(__dirname, 'src-tauri/icons/app-icon-square.png');

async function makeSquare() {
  try {
    // Get image metadata
    const metadata = await sharp(inputPath).metadata();
    const { width, height } = metadata;
    
    // Determine the size of the square (use the larger dimension)
    const size = Math.max(width, height);
    
    // Create a square image with transparent background
    await sharp(inputPath)
      .resize({
        width: size,
        height: size,
        fit: 'contain',
        background: { r: 0, g: 0, b: 0, alpha: 0 }
      })
      .toFile(outputPath);
    
    console.log(`Square icon created at ${outputPath}`);
  } catch (error) {
    console.error('Error creating square icon:', error);
  }
}

makeSquare();
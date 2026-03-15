import matplotlib.pyplot as plt
from skimage import io
from skimage.filters import threshold_otsu
import numpy as np

# Load 16-bit grayscale image (replace with your file path)
# filename = 'test_image.tif'  # Supports TIFF, PNG - use .tif for 16-bit
filename = 'img\\Circle.tif'  # Supports TIFF, PNG - use .tif for 16-bit
image = io.imread(filename, as_gray=True)

# Ensure it's 16-bit uint16
print(f"Image loaded: {filename}")
print(f"Shape: {image.shape}, Dtype: {image.dtype}, Range: [{image.min():.0f}, {image.max():.0f}]")

# Compute Otsu threshold
thresh = threshold_otsu(image)
print(f"Otsu threshold value: {thresh:.2f}")

# Binarize
binary = image > thresh

# Plot results
fig, axes = plt.subplots(ncols=3, figsize=(10, 3.5))
ax = axes.ravel()

ax[0].imshow(image, cmap=plt.cm.gray)
ax[0].set_title('Original 16-bit')
ax[0].axis('off')

ax[1].hist(image.ravel(), bins=1024)  # More bins for 16-bit data
ax[1].set_title('Histogram')
ax[1].axvline(thresh, color='r', linewidth=2)
ax[1].set_xlim(0, image.max())

ax[2].imshow(binary, cmap=plt.cm.gray)
ax[2].set_title(f'Thresholded (T={thresh:.0f})')
ax[2].axis('off')

plt.tight_layout()
plt.show()

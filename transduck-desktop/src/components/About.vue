<template>
  <div class="about-container">
    <div class="card main-card">
      <!-- 页面标题 -->
      <div class="card-header">
        <div class="header-content">
          <i class="bi bi-info-circle header-icon"></i>
          <div>
            <h2>{{ $t('about.title') }}</h2>
            <p class="text-muted mb-0">{{ $t('about.description') }}</p>
          </div>
        </div>
      </div>
      
      <div class="card-body">
        <div class="about-content">
          <div class="links-section">
            <p class="app-description">{{ $t('about.appDescription') }}</p>
            <div class="link-info">
              <div class="link-item">
                <i class="bi bi-globe link-icon"></i>
                <div>
                  <span class="link-label">{{ $t('about.website') }}:</span>
                  <a href="https://transduck.com" target="_blank" class="link-url">https://transduck.com</a>
                </div>
              </div>
              <div class="link-item">
                <i class="bi bi-github link-icon"></i>
                <div>
                  <span class="link-label">GitHub:</span>
                  <a href="https://github.com/dyc87112/transduck-desktop" target="_blank" class="link-url">https://github.com/dyc87112/transduck-desktop</a>
                </div>
              </div>
            </div>
          </div>
          
          <div class="version-info">
            <i class="bi bi-info-circle"></i>
            <span>{{ $t('about.version') }}: {{ pkg.version }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// 导入package.json以获取版本号
import pkg from '../../package.json';
</script>

<style scoped>
@import '../styles/common.css';

.about-container {
  padding: 20px;
  height: 100%;
  background-color: #f9fafb;
}

.about-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.app-description {
  color: #4a5568;
  line-height: 1.6;
  font-size: 1rem;
  margin-bottom: 24px;
}

.links-section {
  background-color: #f8fafc;
  border-radius: 12px;
  padding: 24px;
}

.link-info {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.link-item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px;
  background-color: #ffffff;
  border-radius: 8px;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.05);
  transition: all 0.2s ease;
}

.link-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(67, 97, 238, 0.15);
}

.link-icon {
  font-size: 1.5rem;
  color: #4361ee;
  background-color: rgba(67, 97, 238, 0.1);
  padding: 10px;
  border-radius: 8px;
}

.link-label {
  font-weight: 500;
  color: #4a5568;
  margin-right: 8px;
}

.link-url {
  color: #4361ee;
  word-break: break-all;
  text-decoration: none;
  transition: color 0.2s ease;
}

.link-url:hover {
  color: #3a56d4;
  text-decoration: underline;
}

.version-info {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  color: #6c757d;
  padding: 16px;
  background-color: #ffffff;
  border-radius: 8px;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.05);
}

.version-info i {
  color: #4361ee;
  font-size: 1.1rem;
}

@media (max-width: 768px) {
  .about-container {
    padding: 15px;
  }
  
  .link-item {
    flex-direction: column;
    text-align: center;
    padding: 20px;
  }
  
  .link-icon {
    margin-bottom: 12px;
  }
  
  .version-info {
    flex-direction: column;
    gap: 12px;
    text-align: center;
  }
}
</style>
<template>
  <div class="about-container">
    <!-- 关于信息区 -->
    <div class="about-section">
      <div class="section-header">
        <i class="bi bi-info-circle section-icon"></i>
        <div>
          <h2>{{ $t('about.title') }}</h2>
          <p>{{ $t('about.description') }}</p>
        </div>
      </div>
      
      <div class="about-content">
        
        <div class="links-section">
          <p class="app-description">{{ $t('about.appDescription') }}</p>
          <br>
          <div class="link-info">
            <div class="link-item">
              <i class="bi bi-globe link-icon"></i>
              <div>
                <span class="link-label">{{ $t('about.website') }}:</span>
                <span class="link-url">https://transduck.com</span>
              </div>
            </div>
            <div class="link-item">
              <i class="bi bi-github link-icon"></i>
              <div>
                <span class="link-label">GitHub:</span>
                <span class="link-url">https://github.com/dyc87112/transduck-desktop</span>
              </div>
            </div>
          </div>
        </div>
        
        <div class="version-info">
          <p>{{ $t('about.version') }}: {{ pkg.version }}</p>
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
}

.about-section {
  padding: 25px;
}

.about-content {
  margin-top: 20px;
  display: flex;
  flex-direction: column;
  gap: 30px;
}

.app-description {
  color: #4a5568;
  line-height: 1.6;
  font-size: 1rem;
}

.links-section {
  padding: 20px;
  background-color: #ffffff;
  border-radius: 12px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.05);
}

.link-info {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.link-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background-color: #f8fafc;
  border-radius: 8px;
  transition: all 0.2s ease;
}

.link-item:hover {
  background-color: #f1f5f9;
  transform: translateY(-1px);
}

.link-icon {
  font-size: 1.5rem;
  color: #4361ee;
}

.link-label {
  font-weight: 500;
  color: #4a5568;
  margin-right: 5px;
}

.link-url {
  color: #4361ee;
  word-break: break-all;
  text-decoration: none;
}

.link-url:hover {
  text-decoration: underline;
}

.version-info {
  text-align: center;
  color: #6c757d;
  padding: 16px;
  background-color: #f8fafc;
  border-radius: 8px;
}

@media (max-width: 768px) {
  .about-container {
    padding: 15px;
  }
  
  .about-section {
    padding: 20px;
  }
  
  .link-item {
    flex-direction: column;
    text-align: center;
    padding: 16px;
  }
  
  .link-icon {
    margin-bottom: 8px;
  }
}
</style>
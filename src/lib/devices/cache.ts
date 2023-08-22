import { LRUCache } from 'lru-cache';

const options = {
	max: 100,
    maxSize: 5000,
    sizeCalculation: (value: any, key: any) => {
      return 1
    },
    
	ttl: 1000 * 15
};

export const deviceHeartBeatCache = new LRUCache(options);
